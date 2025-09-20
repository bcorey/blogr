use crate::config::EnvConfig;
use crate::generator::SiteBuilder;
use crate::project::Project;
use crate::utils::Console;
use anyhow::{anyhow, Context, Result};
use git2::{BranchType, Repository, Signature};
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

pub async fn handle_deploy(branch: String, message: Option<String>) -> Result<()> {
    Console::info(&format!("Deploying to GitHub Pages (branch: {branch})"));

    // Check if we're in a blogr project
    let project = Project::find_project()?
        .ok_or_else(|| anyhow!("Not in a blogr project. Run 'blogr init' first."))?;

    // Load configuration and verify GitHub settings
    let config = project.load_config()?;
    let github_config = config.github.as_ref()
        .ok_or_else(|| anyhow!("GitHub configuration not found. Initialize with GitHub integration or configure manually."))?;

    Console::step(1, 6, "Building site...");

    // Build the site first (include drafts: false, future: false for production)
    let temp_output = project.root.join("_site");
    let site_builder = SiteBuilder::new(project.clone(), Some(temp_output.clone()), false, false)?;
    site_builder.build()?;

    Console::step(2, 6, "Checking git status...");

    // Open the git repository
    let repo = Repository::open(&project.root)
        .with_context(|| "Failed to open git repository. Ensure this is a git repository.")?;

    // Check if working directory is clean
    let statuses = repo.statuses(None)?;
    if !statuses.is_empty() {
        Console::warn("Working directory has uncommitted changes. Consider committing them first.");
        println!("Uncommitted changes:");
        for entry in statuses.iter() {
            if let Some(path) = entry.path() {
                println!("  {}", path);
            }
        }

        // Ask user if they want to continue
        print!("Continue with deployment? (y/N): ");
        use std::io::{self, Write};
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        if !input.trim().to_lowercase().starts_with('y') {
            anyhow::bail!("Deployment cancelled");
        }
    }

    Console::step(3, 6, &format!("Preparing {} branch...", branch));

    // Get current branch name
    let head = repo.head()?;
    let current_branch = head.shorthand().unwrap_or("HEAD").to_string();

    // Check if deployment branch exists
    let deploy_branch_exists = repo.find_branch(&branch, BranchType::Local).is_ok();

    if deploy_branch_exists {
        // Checkout existing deployment branch
        checkout_branch(&repo, &branch)?;
    } else {
        // Create orphan branch for GitHub Pages
        create_orphan_branch(&repo, &branch)?;
    }

    Console::step(4, 6, "Copying built files...");

    // Clear the deployment branch (except .git)
    clear_deployment_branch(&project.root)?;

    // Copy built site to deployment branch root
    copy_site_files(&temp_output, &project.root)?;

    // Create CNAME file if custom domain is configured
    let base_url = &config.blog.base_url;
    if let Ok(url) = url::Url::parse(base_url) {
        if let Some(host) = url.host_str() {
            if !host.contains("github.io") {
                let cname_path = project.root.join("CNAME");
                fs::write(cname_path, host)?;
            }
        }
    }

    Console::step(5, 6, "Committing changes...");

    // Stage all files
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    // Create commit
    let deploy_message = message.unwrap_or_else(|| {
        format!(
            "Deploy site - {}",
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
        )
    });

    let signature = get_git_signature()?;
    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    // Check if this is the first commit on this branch
    let parent_commit = repo
        .head()
        .ok()
        .and_then(|head| head.target())
        .and_then(|oid| repo.find_commit(oid).ok());

    let commit_id = if let Some(parent) = parent_commit {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &deploy_message,
            &tree,
            &[&parent],
        )?
    } else {
        repo.commit(
            Some("HEAD"),
            &signature,
            &signature,
            &deploy_message,
            &tree,
            &[],
        )?
    };

    Console::step(6, 6, "Pushing to GitHub...");

    // Push to GitHub
    push_to_github(&repo, &branch, github_config)?;

    // Switch back to original branch
    checkout_branch(&repo, &current_branch)?;

    // Clean up temporary build directory
    if temp_output.exists() {
        fs::remove_dir_all(&temp_output)?;
    }

    println!();
    Console::success("🚀 Site deployed to GitHub Pages!");
    println!("🌐 Your site will be available at:");

    println!("   {}", config.blog.base_url);

    println!("📝 Deployment branch: {}", branch);
    println!("📦 Commit: {}", commit_id);
    println!();

    // Check GitHub Pages deployment status
    Console::info("Checking GitHub Pages deployment status...");
    match check_github_pages_status(&github_config.username, &github_config.repository).await {
        Ok(status) => match status.as_str() {
            "built" => Console::success("GitHub Pages is enabled and working"),
            "building" => Console::info("GitHub Pages is currently building"),
            "errored" => Console::warn("GitHub Pages deployment has errors"),
            _ => Console::info(&format!("GitHub Pages status: {}", status)),
        },
        Err(e) => Console::warn(&format!("Could not check GitHub Pages status: {}", e)),
    }

    println!();
    println!("ℹ️  Note: It may take a few minutes for changes to appear on GitHub Pages");

    Ok(())
}

async fn check_github_pages_status(username: &str, repository: &str) -> Result<String> {
    let github_token = EnvConfig::github_token()
        .ok_or_else(|| anyhow!("GitHub token not found. Set GITHUB_TOKEN environment variable."))?;

    let client = reqwest::Client::new();
    let url = format!(
        "https://api.github.com/repos/{}/{}/pages",
        username, repository
    );

    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", github_token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "blogr-cli")
        .send()
        .await
        .with_context(|| "Failed to check GitHub Pages status")?;

    if response.status() == 404 {
        return Ok("not_configured".to_string());
    }

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("GitHub API error ({status}): {body}");
    }

    let pages_info: serde_json::Value = response
        .json()
        .await
        .with_context(|| "Failed to parse GitHub Pages response")?;

    let status = pages_info
        .get("status")
        .and_then(|s| s.as_str())
        .unwrap_or("unknown")
        .to_string();

    Ok(status)
}

fn checkout_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    let (object, reference) = repo.revparse_ext(branch_name)?;
    repo.checkout_tree(&object, None)?;

    match reference {
        Some(gref) => repo.set_head(gref.name().unwrap()),
        None => repo.set_head_detached(object.id()),
    }?;

    Ok(())
}

fn create_orphan_branch(repo: &Repository, branch_name: &str) -> Result<()> {
    // Create a new orphan branch by creating an empty commit
    let signature = get_git_signature()?;

    // Create an empty tree
    let tree_id = {
        let tree_builder = repo.treebuilder(None)?;
        tree_builder.write()?
    };
    let tree = repo.find_tree(tree_id)?;

    // Create the initial commit on the new branch
    let commit_id = repo.commit(
        Some(&format!("refs/heads/{}", branch_name)),
        &signature,
        &signature,
        "Initial commit for GitHub Pages",
        &tree,
        &[],
    )?;

    // Checkout the new branch
    let commit = repo.find_commit(commit_id)?;
    repo.checkout_tree(commit.as_object(), None)?;
    repo.set_head(&format!("refs/heads/{}", branch_name))?;

    Ok(())
}

fn clear_deployment_branch(project_root: &Path) -> Result<()> {
    for entry in fs::read_dir(project_root)? {
        let entry = entry?;
        let path = entry.path();

        // Skip .git directory
        if path.file_name().unwrap_or_default() == ".git" {
            continue;
        }

        if path.is_dir() {
            fs::remove_dir_all(&path)?;
        } else {
            fs::remove_file(&path)?;
        }
    }

    Ok(())
}

fn copy_site_files(source: &Path, dest: &Path) -> Result<()> {
    for entry in WalkDir::new(source) {
        let entry = entry?;
        let path = entry.path();

        if path == source {
            continue;
        }

        let relative_path = path.strip_prefix(source)?;
        let dest_path = dest.join(relative_path);

        if path.is_dir() {
            fs::create_dir_all(&dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::copy(path, dest_path)?;
        }
    }

    Ok(())
}

fn get_git_signature() -> Result<Signature<'static>> {
    let name = EnvConfig::git_author_name().unwrap_or_else(|| "Blogr".to_string());
    let email = EnvConfig::git_author_email().unwrap_or_else(|| "blogr@example.com".to_string());

    Signature::now(&name, &email).with_context(|| "Failed to create git signature")
}

fn push_to_github(
    repo: &Repository,
    branch: &str,
    github_config: &crate::config::GitHubConfig,
) -> Result<()> {
    let github_token = EnvConfig::github_token()
        .ok_or_else(|| anyhow!("GitHub token not found. Set GITHUB_TOKEN environment variable."))?;

    // Create remote URL with token
    let remote_url = format!(
        "https://{}@github.com/{}/{}.git",
        github_token, github_config.username, github_config.repository
    );

    // Get or create remote
    let mut remote = match repo.find_remote("origin") {
        Ok(remote) => remote,
        Err(_) => repo.remote("origin", &remote_url)?,
    };

    // Push the branch
    let refspec = format!("refs/heads/{}:refs/heads/{}", branch, branch);
    remote.push(&[&refspec], None)?;

    Ok(())
}

use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::config::EnvConfig;
use crate::project::{AutoInit, Project};
use crate::utils::{Console, Utils};

pub async fn handle_init(
    name: Option<String>,
    path: Option<PathBuf>,
    github_username: Option<String>,
    github_repo: Option<String>,
    no_github: bool,
) -> Result<()> {
    Console::info("Initializing new Blogr project...");
    println!();

    // Determine project path
    let project_path = path.unwrap_or_else(|| std::env::current_dir().unwrap());

    // Check if we're already in a Blogr project
    if Project::is_in_project() {
        anyhow::bail!("Already inside a Blogr project. Cannot initialize a new project here.");
    }

    // Interactive mode if no name provided
    let project_name = match name {
        Some(name) => name,
        None => {
            use std::io::{self, Write};
            print!("Project title: ");
            io::stdout().flush()?;
            let mut input = String::new();
            io::stdin().read_line(&mut input)?;
            let input = input.trim();
            if input.is_empty() {
                anyhow::bail!("Project title cannot be empty");
            }
            input.to_string()
        }
    };

    // Get author name
    let default_author = EnvConfig::git_author_name().unwrap_or_else(|| "Anonymous".to_string());
    let author = {
        use std::io::{self, Write};
        print!("Author name [{}]: ", default_author);
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.is_empty() {
            default_author
        } else {
            input.to_string()
        }
    };

    // Get description
    let description = {
        use std::io::{self, Write};
        print!("Blog description: ");
        io::stdout().flush()?;
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        if input.is_empty() {
            format!("A blog by {}", author)
        } else {
            input.to_string()
        }
    };

    // GitHub integration
    let (final_github_username, final_github_repo) = if no_github {
        (None, None)
    } else {
        let username = match github_username {
            Some(username) => {
                if !Utils::is_valid_github_username(&username) {
                    anyhow::bail!("Invalid GitHub username: {}", username);
                }
                Some(username)
            }
            None => {
                let default_username = EnvConfig::github_username();
                use std::io::{self, Write};

                if let Some(ref default) = default_username {
                    print!("GitHub username [{}] (press Enter to skip): ", default);
                } else {
                    print!("GitHub username (press Enter to skip): ");
                }
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim();

                if input.is_empty() {
                    default_username
                } else if Utils::is_valid_github_username(input) {
                    Some(input.to_string())
                } else {
                    Console::warn(&format!(
                        "Invalid GitHub username '{}', skipping GitHub integration",
                        input
                    ));
                    None
                }
            }
        };

        let repo = if username.is_some() {
            match github_repo {
                Some(repo) => {
                    if !Utils::is_valid_github_repo_name(&repo) {
                        anyhow::bail!("Invalid GitHub repository name: {}", repo);
                    }
                    Some(repo)
                }
                None => {
                    let default_repo = Utils::slugify(&project_name);
                    use std::io::{self, Write};
                    print!("GitHub repository name [{}]: ", default_repo);
                    io::stdout().flush()?;

                    let mut input = String::new();
                    io::stdin().read_line(&mut input)?;
                    let input = input.trim();

                    let repo_name = if input.is_empty() {
                        default_repo
                    } else {
                        input.to_string()
                    };

                    if Utils::is_valid_github_repo_name(&repo_name) {
                        Some(repo_name)
                    } else {
                        Console::warn(&format!(
                            "Invalid repository name '{}', skipping GitHub integration",
                            repo_name
                        ));
                        None
                    }
                }
            }
        } else {
            None
        };

        (username, repo)
    };

    Console::step(1, 5, "Creating project structure...");

    // Initialize the project
    let project = Project::init(
        &project_path,
        project_name.clone(),
        author.clone(),
        description.clone(),
        final_github_username.clone(),
        final_github_repo.clone(),
    )
    .with_context(|| "Failed to initialize project")?;

    Console::step(2, 5, "Initializing Git repository...");

    // Initialize git repository
    if Utils::is_git_available() {
        if let Err(e) = Utils::init_git_repo(&project.root) {
            Console::warn(&format!("Failed to initialize git repository: {}", e));
        } else {
            // Create initial commit
            if let Err(e) = Utils::git_add_all(&project.root) {
                Console::warn(&format!("Failed to stage files: {}", e));
            } else if let Err(e) =
                Utils::git_initial_commit(&project.root, "feat: initial commit with Blogr setup")
            {
                Console::warn(&format!("Failed to create initial commit: {}", e));
            }
        }
    } else {
        Console::warn("Git not available, skipping repository initialization");
    }

    Console::step(3, 5, "Setting up theme...");

    // Theme is already set up in project initialization (minimal-retro by default)
    Console::success("Minimal Retro theme configured");

    // GitHub repository creation
    if let (Some(username), Some(repo)) = (&final_github_username, &final_github_repo) {
        Console::step(4, 5, "Creating GitHub repository...");

        match create_github_repository(username, repo, &description).await {
            Ok(()) => {
                Console::success(&format!("GitHub repository created: {}/{}", username, repo));

                // Set up git remote
                let remote_url = format!("https://github.com/{}/{}.git", username, repo);
                if let Err(e) = Utils::git_set_remote(&project.root, &remote_url) {
                    Console::warn(&format!("Failed to set git remote: {}", e));
                } else {
                    // Push initial commit
                    if let Err(e) = Utils::git_push(&project.root, "main") {
                        Console::warn(&format!("Failed to push to GitHub: {}", e));
                        Console::info("You can push later with: git push -u origin main");
                    } else {
                        Console::success("Initial commit pushed to GitHub");
                    }
                }
            }
            Err(e) => {
                Console::warn(&format!("Failed to create GitHub repository: {}", e));
                Console::info(&format!(
                    "You can create it manually at: https://github.com/new"
                ));
                if let (Some(username), Some(repo)) = (&final_github_username, &final_github_repo) {
                    Console::info(&format!(
                        "Repository URL: https://github.com/{}/{}",
                        username, repo
                    ));
                }
            }
        }
    }

    Console::step(5, 5, "Finalizing setup...");

    // Validate the project
    let issues = project.validate()?;
    if !issues.is_empty() {
        Console::warn("Project validation found some issues:");
        for issue in issues {
            println!("  - {}", issue);
        }
    } else {
        Console::success("Project validation passed");
    }

    println!();
    Console::success(&format!("🎉 Successfully initialized '{}'!", project_name));
    println!();

    // Show next steps
    println!("📁 Project created in: {}", project.root.display());
    println!();
    println!("🚀 Next steps:");
    println!("  1. cd {}", project.root.display());
    println!("  2. blogr serve          # Start development server");
    println!("  3. blogr new \"My Post\" # Create your first post");
    println!("  4. blogr build          # Build the static site");
    println!();

    if let (Some(username), Some(repo)) = (&final_github_username, &final_github_repo) {
        println!("🌐 Your blog will be available at:");
        println!("   https://{}.github.io/{}", username, repo);
        println!();
    }

    println!("📚 Learn more:");
    println!("  - Run 'blogr --help' for available commands");
    println!("  - Edit 'blogr.toml' to customize your blog");
    println!("  - Add posts to the 'posts/' directory");
    println!();

    Ok(())
}

async fn create_github_repository(
    username: &str,
    repo_name: &str,
    description: &str,
) -> Result<()> {
    let github_token = EnvConfig::github_token().ok_or_else(|| {
        anyhow::anyhow!("GitHub token not found. Set GITHUB_TOKEN environment variable.")
    })?;

    let client = reqwest::Client::new();
    let repo_data = serde_json::json!({
        "name": repo_name,
        "description": description,
        "private": false,
        "auto_init": false,
        "gitignore_template": null,
        "license_template": null
    });

    let response = client
        .post("https://api.github.com/user/repos")
        .header("Authorization", format!("Bearer {}", github_token))
        .header("Accept", "application/vnd.github.v3+json")
        .header("User-Agent", "blogr-cli")
        .json(&repo_data)
        .send()
        .await
        .with_context(|| "Failed to send request to GitHub API")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();

        if status == 422 {
            anyhow::bail!(
                "Repository '{}' already exists or name is invalid",
                repo_name
            );
        } else if status == 401 {
            anyhow::bail!("Authentication failed. Check your GitHub token.");
        } else {
            anyhow::bail!("GitHub API error ({}): {}", status, body);
        }
    }

    Ok(())
}

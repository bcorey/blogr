//! Newsletter command handlers

use anyhow::{Context, Result};

use crate::newsletter::NewsletterManager;
use crate::project::Project;

/// Handle the fetch-subscribers command
pub async fn handle_fetch_subscribers(interactive: bool) -> Result<()> {
    // Find the current project
    let project = Project::find_project()?
        .ok_or_else(|| anyhow::anyhow!("Not in a blogr project. Run 'blogr init' first."))?;

    // Load project configuration
    let config = project
        .load_config()
        .context("Failed to load project configuration")?;

    // Create newsletter manager
    let mut newsletter_manager = NewsletterManager::new(config, &project.root)
        .context("Failed to initialize newsletter manager")?;

    // Check if newsletter is enabled
    if !newsletter_manager.is_enabled() {
        println!("Newsletter functionality is not enabled.");
        println!("To enable it, add the following to your blogr.toml:");
        println!();
        println!("[newsletter]");
        println!("enabled = true");
        println!("subscribe_email = \"subscribe@yourdomain.com\"");
        println!("sender_name = \"Your Blog Name\"");
        println!();
        println!("You'll also need to configure IMAP settings:");
        println!();
        println!("[newsletter.imap]");
        println!("server = \"imap.gmail.com\"");
        println!("port = 993");
        println!("username = \"subscribe@yourdomain.com\"");
        println!();
        println!("And set the NEWSLETTER_IMAP_PASSWORD environment variable.");
        return Ok(());
    }

    // Print current configuration status
    newsletter_manager
        .print_status()
        .context("Failed to print newsletter status")?;

    println!();

    // If interactive mode, offer to set up configuration
    if interactive {
        match newsletter_manager.get_imap_config()? {
            Some(_) => {
                println!("IMAP configuration found. Proceeding with email fetch...");
            }
            None => {
                println!("No IMAP configuration found.");
                if prompt_yes_no("Would you like to set up IMAP configuration now?")? {
                    let imap_config = crate::newsletter::config::setup_imap_config()?;
                    println!("IMAP configuration created. Please add it to your blogr.toml:");
                    println!();
                    println!("[newsletter.imap]");
                    println!("server = \"{}\"", imap_config.server);
                    println!("port = {}", imap_config.port);
                    println!("username = \"{}\"", imap_config.username);
                    println!();
                    println!("Also set the NEWSLETTER_IMAP_PASSWORD environment variable and run the command again.");
                    return Ok(());
                }
            }
        }
    }

    // Attempt to fetch subscribers
    match newsletter_manager.fetch_subscribers(interactive) {
        Ok(()) => {
            println!("✓ Newsletter subscriber fetch completed successfully.");

            // Show updated statistics
            println!("\nUpdated subscriber statistics:");
            let total = newsletter_manager.database().get_subscriber_count(None)?;
            let pending = newsletter_manager.database().get_subscriber_count(Some(
                crate::newsletter::database::SubscriberStatus::Pending,
            ))?;
            let approved = newsletter_manager.database().get_subscriber_count(Some(
                crate::newsletter::database::SubscriberStatus::Approved,
            ))?;

            println!("  Total: {}", total);
            println!("  Pending: {}", pending);
            println!("  Approved: {}", approved);

            if pending > 0 {
                println!("\nNext steps:");
                println!("  1. Run 'blogr newsletter approve' to review pending subscribers");
                println!("  2. Run 'blogr newsletter list' to see all subscribers");
            }
        }
        Err(e) => {
            eprintln!("Failed to fetch subscribers: {}", e);

            // Provide helpful troubleshooting information
            println!("\nTroubleshooting tips:");
            println!("1. Verify your IMAP credentials are correct");
            println!("2. Check that NEWSLETTER_IMAP_PASSWORD environment variable is set");
            println!("3. Ensure your email provider allows IMAP access");
            println!("4. For Gmail, you may need to use an App Password instead of your regular password");
            println!("5. Check your firewall and network connectivity");

            return Err(e);
        }
    }

    Ok(())
}

/// Prompt user for yes/no input
fn prompt_yes_no(question: &str) -> Result<bool> {
    use std::io::{self, Write};

    loop {
        print!("{} (y/n): ", question);
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim().to_lowercase();

        match input.as_str() {
            "y" | "yes" => return Ok(true),
            "n" | "no" => return Ok(false),
            _ => println!("Please enter 'y' or 'n'"),
        }
    }
}

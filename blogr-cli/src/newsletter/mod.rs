//! Newsletter module for handling email-based subscriptions
//!
//! This module provides functionality for:
//! - Fetching subscription emails via IMAP
//! - Parsing and extracting subscriber information
//! - Managing subscriber database
//! - Email composition and sending

pub mod config;
pub mod database;
pub mod fetcher;
pub mod ui;

pub use config::NewsletterManager;
pub use database::{NewsletterDatabase, Subscriber, SubscriberStatus};
pub use ui::{ApprovalApp, ApprovalResult};

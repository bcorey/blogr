//! Database operations for newsletter subscribers

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection, Row};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SubscriberStatus {
    Pending,
    Approved,
    Declined,
}

impl std::fmt::Display for SubscriberStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriberStatus::Pending => write!(f, "pending"),
            SubscriberStatus::Approved => write!(f, "approved"),
            SubscriberStatus::Declined => write!(f, "declined"),
        }
    }
}

impl std::str::FromStr for SubscriberStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "pending" => Ok(SubscriberStatus::Pending),
            "approved" => Ok(SubscriberStatus::Approved),
            "declined" => Ok(SubscriberStatus::Declined),
            _ => Err(anyhow::anyhow!("Invalid subscriber status: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscriber {
    pub id: Option<i64>,
    pub email: String,
    pub status: SubscriberStatus,
    pub subscribed_at: DateTime<Utc>,
    pub approved_at: Option<DateTime<Utc>>,
    pub source_email_id: Option<String>,
    pub notes: Option<String>,
}

impl Subscriber {
    pub fn new(email: String, source_email_id: Option<String>) -> Self {
        Self {
            id: None,
            email,
            status: SubscriberStatus::Pending,
            subscribed_at: Utc::now(),
            approved_at: None,
            source_email_id,
            notes: None,
        }
    }

    #[allow(dead_code)]
    pub fn approve(&mut self) {
        self.status = SubscriberStatus::Approved;
        self.approved_at = Some(Utc::now());
    }

    #[allow(dead_code)]
    pub fn decline(&mut self) {
        self.status = SubscriberStatus::Declined;
        self.approved_at = Some(Utc::now());
    }
}

pub struct NewsletterDatabase {
    conn: Connection,
}

impl NewsletterDatabase {
    /// Create or open the newsletter database
    pub fn open<P: AsRef<Path>>(path: P) -> Result<Self> {
        let conn = Connection::open(&path)
            .with_context(|| format!("Failed to open database at {}", path.as_ref().display()))?;

        let mut db = Self { conn };
        db.initialize()?;
        Ok(db)
    }

    /// Initialize the database schema
    fn initialize(&mut self) -> Result<()> {
        self.conn
            .execute_batch(
                r#"
            CREATE TABLE IF NOT EXISTS subscribers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                email TEXT UNIQUE NOT NULL,
                status TEXT NOT NULL CHECK (status IN ('pending', 'approved', 'declined')),
                subscribed_at DATETIME DEFAULT CURRENT_TIMESTAMP,
                approved_at DATETIME,
                source_email_id TEXT,
                notes TEXT
            );

            CREATE INDEX IF NOT EXISTS idx_subscribers_status ON subscribers(status);
            CREATE INDEX IF NOT EXISTS idx_subscribers_email ON subscribers(email);
            CREATE INDEX IF NOT EXISTS idx_subscribers_subscribed_at ON subscribers(subscribed_at);
            "#,
            )
            .context("Failed to initialize database schema")?;

        Ok(())
    }

    /// Add a new subscriber to the database
    pub fn add_subscriber(&mut self, subscriber: &Subscriber) -> Result<i64> {
        let mut stmt = self.conn.prepare(
            "INSERT INTO subscribers (email, status, subscribed_at, source_email_id, notes) 
             VALUES (?1, ?2, ?3, ?4, ?5)",
        )?;

        let id = stmt.insert(params![
            subscriber.email,
            subscriber.status.to_string(),
            subscriber
                .subscribed_at
                .format("%Y-%m-%d %H:%M:%S%.3f")
                .to_string(),
            subscriber.source_email_id,
            subscriber.notes,
        ])?;

        Ok(id)
    }

    /// Update subscriber status
    pub fn update_subscriber_status(&mut self, id: i64, status: SubscriberStatus) -> Result<()> {
        let approved_at = if matches!(
            status,
            SubscriberStatus::Approved | SubscriberStatus::Declined
        ) {
            Some(Utc::now().format("%Y-%m-%d %H:%M:%S%.3f").to_string())
        } else {
            None
        };

        self.conn.execute(
            "UPDATE subscribers SET status = ?1, approved_at = ?2 WHERE id = ?3",
            params![status.to_string(), approved_at, id],
        )?;

        Ok(())
    }

    /// Get all subscribers with optional status filter
    pub fn get_subscribers(
        &self,
        status_filter: Option<SubscriberStatus>,
    ) -> Result<Vec<Subscriber>> {
        let (query, params): (String, Vec<String>) = match status_filter {
            Some(status) => (
                "SELECT id, email, status, subscribed_at, approved_at, source_email_id, notes 
                 FROM subscribers WHERE status = ?1 ORDER BY subscribed_at DESC"
                    .to_string(),
                vec![status.to_string()],
            ),
            None => (
                "SELECT id, email, status, subscribed_at, approved_at, source_email_id, notes 
                 FROM subscribers ORDER BY subscribed_at DESC"
                    .to_string(),
                vec![],
            ),
        };

        let mut stmt = self.conn.prepare(&query)?;
        let subscriber_iter = stmt.query_map(rusqlite::params_from_iter(params.iter()), |row| {
            self.row_to_subscriber(row)
        })?;

        let mut subscribers = Vec::new();
        for subscriber in subscriber_iter {
            subscribers.push(subscriber?);
        }

        Ok(subscribers)
    }

    /// Get subscriber by email
    pub fn get_subscriber_by_email(&self, email: &str) -> Result<Option<Subscriber>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, email, status, subscribed_at, approved_at, source_email_id, notes 
             FROM subscribers WHERE email = ?1",
        )?;

        let result = stmt.query_row(params![email], |row| self.row_to_subscriber(row));

        match result {
            Ok(subscriber) => Ok(Some(subscriber)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    /// Remove subscriber by email
    pub fn remove_subscriber(&mut self, email: &str) -> Result<bool> {
        let rows_affected = self
            .conn
            .execute("DELETE FROM subscribers WHERE email = ?1", params![email])?;

        Ok(rows_affected > 0)
    }

    /// Get subscriber count by status
    pub fn get_subscriber_count(&self, status: Option<SubscriberStatus>) -> Result<i64> {
        let (query, params): (String, Vec<String>) = match status {
            Some(status) => (
                "SELECT COUNT(*) FROM subscribers WHERE status = ?1".to_string(),
                vec![status.to_string()],
            ),
            None => ("SELECT COUNT(*) FROM subscribers".to_string(), vec![]),
        };

        let mut stmt = self.conn.prepare(&query)?;
        let count: i64 =
            stmt.query_row(rusqlite::params_from_iter(params.iter()), |row| row.get(0))?;

        Ok(count)
    }

    /// Check if email already exists
    pub fn email_exists(&self, email: &str) -> Result<bool> {
        let mut stmt = self
            .conn
            .prepare("SELECT 1 FROM subscribers WHERE email = ?1")?;
        let result = stmt.query_row(params![email], |_| Ok(()));

        match result {
            Ok(_) => Ok(true),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(e.into()),
        }
    }

    /// Helper function to convert database row to Subscriber
    fn row_to_subscriber(&self, row: &Row) -> rusqlite::Result<Subscriber> {
        let subscribed_at_str: String = row.get(3)?;
        let approved_at_str: Option<String> = row.get(4)?;

        let subscribed_at =
            chrono::NaiveDateTime::parse_from_str(&subscribed_at_str, "%Y-%m-%d %H:%M:%S")
                .or_else(|_| {
                    chrono::NaiveDateTime::parse_from_str(
                        &subscribed_at_str,
                        "%Y-%m-%d %H:%M:%S%.3f",
                    )
                })
                .map_err(|_e| {
                    rusqlite::Error::InvalidColumnType(
                        3,
                        "subscribed_at".to_string(),
                        rusqlite::types::Type::Text,
                    )
                })?
                .and_utc();

        let approved_at = if let Some(approved_str) = approved_at_str {
            Some(
                chrono::NaiveDateTime::parse_from_str(&approved_str, "%Y-%m-%d %H:%M:%S")
                    .or_else(|_| {
                        chrono::NaiveDateTime::parse_from_str(
                            &approved_str,
                            "%Y-%m-%d %H:%M:%S%.3f",
                        )
                    })
                    .map_err(|_e| {
                        rusqlite::Error::InvalidColumnType(
                            4,
                            "approved_at".to_string(),
                            rusqlite::types::Type::Text,
                        )
                    })?
                    .and_utc(),
            )
        } else {
            None
        };

        let status_str: String = row.get(2)?;
        let status = status_str.parse().map_err(|_| {
            rusqlite::Error::InvalidColumnType(2, "status".to_string(), rusqlite::types::Type::Text)
        })?;

        Ok(Subscriber {
            id: Some(row.get(0)?),
            email: row.get(1)?,
            status,
            subscribed_at,
            approved_at,
            source_email_id: row.get(5)?,
            notes: row.get(6)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_database_operations() -> Result<()> {
        let temp_file = NamedTempFile::new()?;
        let mut db = NewsletterDatabase::open(temp_file.path())?;

        // Test adding subscriber
        let subscriber =
            Subscriber::new("test@example.com".to_string(), Some("msg-123".to_string()));
        let id = db.add_subscriber(&subscriber)?;
        assert!(id > 0);

        // Test getting subscriber by email
        let retrieved = db.get_subscriber_by_email("test@example.com")?;
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.email, "test@example.com");
        assert_eq!(retrieved.status, SubscriberStatus::Pending);

        // Test updating status
        db.update_subscriber_status(id, SubscriberStatus::Approved)?;
        let updated = db.get_subscriber_by_email("test@example.com")?.unwrap();
        assert_eq!(updated.status, SubscriberStatus::Approved);
        assert!(updated.approved_at.is_some());

        // Test getting subscribers by status
        let approved_subscribers = db.get_subscribers(Some(SubscriberStatus::Approved))?;
        assert_eq!(approved_subscribers.len(), 1);

        let pending_subscribers = db.get_subscribers(Some(SubscriberStatus::Pending))?;
        assert_eq!(pending_subscribers.len(), 0);

        // Test subscriber count
        let total_count = db.get_subscriber_count(None)?;
        assert_eq!(total_count, 1);

        let approved_count = db.get_subscriber_count(Some(SubscriberStatus::Approved))?;
        assert_eq!(approved_count, 1);

        // Test email exists
        assert!(db.email_exists("test@example.com")?);
        assert!(!db.email_exists("nonexistent@example.com")?);

        // Test removing subscriber
        assert!(db.remove_subscriber("test@example.com")?);
        assert!(!db.email_exists("test@example.com")?);

        Ok(())
    }
}

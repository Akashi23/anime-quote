use crate::db;
use crate::models::quote::Quote;
use serde::{Deserialize, Serialize};
use std::time::{self, SystemTime};
use tokio_postgres::Error;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub password: String,
    created_at: time::SystemTime,
    updated_at: time::SystemTime,
}

impl User {
    pub async fn init() -> Result<(), Error> {
        let client = db::connect().await?();
        client
            .execute(
                "CREATE TABLE IF NOT EXISTS anime_quote.users
            (
                id SERIAL PRIMARY KEY,
                username VARCHAR(255) NOT NULL,
                password VARCHAR(255) NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );",
                &[],
            )
            .await?;

        Ok(())
    }

    pub fn default() -> Self {
        User {
            id: -1,
            username: String::from(""),
            password: String::from(""),
            created_at: time::SystemTime::now(),
            updated_at: time::SystemTime::now(),
        }
    }

    pub async fn new(username: String, password: String) -> Result<Self, Error> {
        let client = db::connect().await?();
        client
            .execute(
                "INSERT INTO anime_quote.users 
                    (username, password, created_at, updated_at) 
                    VALUES ($1, $2, $3, $4);",
                &[&username, &password, &SystemTime::now(), &SystemTime::now()],
            )
            .await?;

        Ok(User {
            id: -1,
            username,
            password,
            created_at: time::SystemTime::now(),
            updated_at: time::SystemTime::now(),
        })
    }

    pub async fn find_by_id(id: i32) -> Result<Self, Error> {
        let client = db::connect().await?();
        let row = client
            .query_one("SELECT * FROM anime_quote.users WHERE id = $1", &[&id])
            .await?;

        Ok(User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        })
    }

    pub async fn find_by_username(username: String) -> Result<Self, Error> {
        let client = db::connect().await?();
        let row = client
            .query_one(
                "SELECT * FROM anime_quote.users WHERE username = $1",
                &[&username],
            )
            .await?;
        format!("{:?}", row);
        Ok(User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        })
    }

    pub async fn update_password(&mut self, password: String) -> Result<(), Error> {
        let client = db::connect().await?();
        client
            .execute(
                "UPDATE anime_quote.users SET password = $1 WHERE id = $2",
                &[&password, &self.id],
            )
            .await?;

        self.password = password;
        Ok(())
    }

    pub async fn delete(&self) -> Result<(), Error> {
        let client = db::connect().await?();
        client
            .execute("DELETE FROM anime_quote.users WHERE id = $1", &[&self.id])
            .await?;
        Ok(())
    }

    pub async fn find() -> Result<Vec<Self>, Error> {
        let client = db::connect().await?();
        let rows = client.query("SELECT * FROM anime_quote.users", &[]).await?;

        let mut users = Vec::new();
        for row in rows {
            users.push(User {
                id: row.get(0),
                username: row.get(1),
                password: row.get(2),
                created_at: row.get(3),
                updated_at: row.get(4),
            });
        }

        Ok(users)
    }

    pub async fn quotes(&self) -> Result<Vec<Quote>, Error> {
        let quotes = Quote::find_by_user_id(self.id).await?;
        Ok(quotes)
    }
}

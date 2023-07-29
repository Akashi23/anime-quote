use std::time::SystemTime;

use tokio_postgres::Error;

use crate::db;

pub struct Quote {
    pub id: i32,
    pub user_id: i32,
    pub text: String,
}

impl Quote {
    pub async fn init() -> Result<(), Error> {
        let client = db::connect().await?();
        client
            .execute(
                "CREATE TABLE IF NOT EXISTS anime_quote.quotes
            (
                id SERIAL PRIMARY KEY,
                user_id INT NOT NULL REFERENCES anime_quote.users(id),
                text VARCHAR(255) NOT NULL,
                created_at TIMESTAMP NOT NULL,
                updated_at TIMESTAMP NOT NULL
            );",
                &[],
            )
            .await?;

        Ok(())
    }

    pub async fn new(user_id: i32, text: String) -> Result<Self, Error> {
        let client = db::connect().await?();
        client
            .execute(
                "INSERT INTO anime_quote.quotes 
                    (user_id, text, created_at, updated_at) 
                    VALUES ($1, $2, $3, $4);",
                &[&user_id, &text, &SystemTime::now(), &SystemTime::now()],
            )
            .await?;

        Ok(Quote {
            id: -1,
            user_id,
            text,
        })
    }

    pub async fn find_by_user_id(user_id: i32) -> Result<Vec<Self>, Error> {
        let client = db::connect().await?();
        let rows = client
            .query(
                "SELECT * FROM anime_quote.quotes WHERE user_id = $1",
                &[&user_id],
            )
            .await?;

        let mut quotes = Vec::new();
        for row in rows {
            quotes.push(Quote {
                id: row.get(0),
                user_id: row.get(1),
                text: row.get(2),
            });
        }

        Ok(quotes)
    }

    pub async fn find_by_id(id: i32) -> Result<Self, Error> {
        let client = db::connect().await?();
        let row = client
            .query_one("SELECT * FROM anime_quote.quotes WHERE id = $1", &[&id])
            .await?;
        Ok(Quote {
            id: row.get(0),
            user_id: row.get(1),
            text: row.get(2),
        })
    }

    pub async fn update(&mut self, text: String) -> Result<(), Error> {
        let client = db::connect().await?();
        client
            .execute(
                "UPDATE anime_quote.quotes SET text = $1, updated_at = $2 WHERE id = $3",
                &[&text, &SystemTime::now(), &self.id],
            )
            .await?;
        self.text = text;
        Ok(())
    }

    pub async fn delete(&self) -> Result<(), Error> {
        let client = db::connect().await?();
        client
            .execute("DELETE FROM anime_quote.quotes WHERE id = $1", &[&self.id])
            .await?;

        Ok(())
    }

    pub async fn find() -> Result<Vec<Self>, Error> {
        let client = db::connect().await?();
        let rows = client
            .query("SELECT * FROM anime_quote.quotes", &[])
            .await?;

        let mut quotes = Vec::new();
        for row in rows {
            quotes.push(Quote {
                id: row.get(0),
                user_id: row.get(1),
                text: row.get(2),
            });
        }

        Ok(quotes)
    }
}

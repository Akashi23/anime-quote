use std::time::SystemTime;

use tokio_postgres::Error;

use crate::db;

pub struct Quote {
    pub id: i32,
    pub user_id: i32,
    pub quote: String,
    pub anime: String,
    pub character: String,
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

    pub async fn new(user_id: i32, quote: String, anime: String, character: String) -> Result<Self, Error> {
        let client = db::connect().await?();
        client
            .execute(
                "INSERT INTO anime_quote.quotes 
                    (user_id, quote, anime, character, created_at, updated_at)
                    VALUES ($1, $2, $3, $4, $5, $6);",
                &[&user_id, &quote, &anime, &character, &SystemTime::now(), &SystemTime::now()],
            )
            .await?;

        Ok(Quote {
            id: -1,
            user_id,
            quote,
            anime,
            character,
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
                quote: row.get(2),
                anime: row.get(3),
                character: row.get(4),
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
            quote: row.get(2),
            anime: row.get(3),
            character: row.get(4),
        })
    }

    pub async fn update(&mut self, quote: String, anime: String, character: String) -> Result<(), Error> {
        let client = db::connect().await?();
        client
            .execute(
                "UPDATE anime_quote.quotes SET quote = $1, anime = $2, character = $3, updated_at = $4 WHERE id = $5",
                &[&quote, &anime, &character, &SystemTime::now(), &self.id],
            )
            .await?;
        self.quote = quote;
        self.anime = anime;
        self.character = character;
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
                quote: row.get(2),
                anime: row.get(3),
                character: row.get(4),
            });
        }

        Ok(quotes)
    }
}

use crate::db;

struct Quote {
    user_id: i32,
    text: String,
}

impl Quote {
    pub fn init() {
        let client = db::connect().await?;
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
            )
            .await?;
    }

    pub fn new(user_id: i32, text: String) -> Self {
        let client = db::connect().await?;
        client
            .execute(
                "INSERT INTO anime_quote.quotes 
                    (user_id, text, created_at, updated_at) 
                    VALUES ($1, $2, $3, $4);",
                &[&user_id, &text, &SystemTime::now(), &SystemTime::now()],
            )
            .await?;
        Quote {
            user_id,
            text,
        }
    }

    pub fn find_by_user_id(user_id: i32) -> Vec<Self> {
        let client = db::connect().await?;
        let rows = client
            .query(
                "SELECT * FROM anime_quote.quotes WHERE user_id = $1",
                &[&user_id],
            )
            .await?;
        
        let mut quotes = Vec::new();
        for row in rows {
            quotes.push(Quote {
                user_id: row.get(0),
                text: row.get(1),
            });
        }

        quotes
    }

    pub fn find_by_id(id: i32) -> Self {
        let client = db::connect().await?;
        let row = client
            .query_one(
                "SELECT * FROM anime_quote.quotes WHERE id = $1",
                &[&id],
            )
            .await?;
        Quote {
            user_id: row.get(0),
            text: row.get(1),
        }
    }

    pub fn update(&mut self, text: String) {
        let client = db::connect().await?;
        client
            .execute(
                "UPDATE anime_quote.quotes SET text = $1, updated_at = $2 WHERE id = $3",
                &[&text, &SystemTime::now(), &self.id],
            )
            .await?;
        self.text = text;
    }

    pub fn delete(&self) {
        let client = db::connect().await?;
        client
            .execute(
                "DELETE FROM anime_quote.quotes WHERE id = $1",
                &[&self.id],
            )
            .await?;
    }

    pub fn find() -> Vec<Self> {
        let client = db::connect().await?;
        let rows = client
            .query(
                "SELECT * FROM anime_quote.quotes",
                &[],
            )
            .await?;
        
        let mut quotes = Vec::new();
        for row in rows {
            quotes.push(Quote {
                user_id: row.get(0),
                text: row.get(1),
            });
        }

        quotes
    }
}
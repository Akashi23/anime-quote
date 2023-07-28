use crate::db;
use time::SystemTime;

struct User {
    id: i32,
    username: String,
    password: String,
    created_at: time::SystemTime,
    updated_at: time::SystemTime,
}

impl User {
    pub fn init() {
        let client = db::connect().await?;
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
            )
            .await?;
    }

    pub fn new(username: String, password: String) -> Self {
        let client = db::connect().await?;
        client
            .execute(
                "INSERT INTO anime_quote.users 
                    (username, password, created_at, updated_at) 
                    VALUES ($1, $2, $3, $4);",
                &[&username, &password, &SystemTime::now(), &SystemTime::now()],
            )
            .await?;
        User {
            id: None,
            username,
            password,
            created_at: time::SystemTime::now(),
            updated_at: time::SystemTime::now(),
        }
    }

    pub fn find_by_username(username: String) -> Self {
        let client = db::connect().await?;
        let row = client
            .query_one(
                "SELECT * FROM anime_quote.users WHERE username = $1",
                &[&username],
            )
            .await?;
        User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        }
    }

    pub fn find_by_id(id: i32) -> Self {
        let client = db::connect().await?;
        let row = client
            .query_one("SELECT * FROM anime_quote.users WHERE id = $1", &[&id])
            .await?;
        User {
            id: row.get(0),
            username: row.get(1),
            password: row.get(2),
            created_at: row.get(3),
            updated_at: row.get(4),
        }
    }

    pub fn update_password(&mut self, password: String) {
        let client = db::connect().await?;
        client
            .execute(
                "UPDATE anime_quote.users SET password = $1 WHERE id = $2",
                &[&password, &self.id],
            )
            .await?;
        self.password = password;
    }

    pub fn delete(&self) {
        let client = db::connect().await?;
        client
            .execute("DELETE FROM anime_quote.users WHERE id = $1", &[&self.id])
            .await?;
    }

    pub fn find() -> Vec<Self> {
        let client = db::connect().await?;
        let rows = client
            .query("SELECT * FROM anime_quote.users", &[])
            .await?;

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

        users
    }

    pub fn quotes(&self) -> Vec<Quote> {
        Quote::find_by_user_id(self.id)
    }
}

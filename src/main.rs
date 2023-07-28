mod db;
mod models;
mod services; // Import the db module

use tokio_postgres::Error;
use db::connect;

#[tokio::main] // By default, tokio_postgres uses the tokio crate as its runtime.
async fn main() -> Result<(), Error> {
    // Connect to the database.
    let connection = connect().await?;

    let client = connection();
    let client2 = connection();

    // Now we can execute a simple statement that just returns its parameter.
    let rows = client
        .query("SELECT $1::TEXT", &[&"hello world"])
        .await?;

    let rows2 = client2
        .query("SELECT $1::TEXT", &[&"hellworld"])
        .await?;

    // And then check that we got back the same string we sent over.
    let value: &str = rows[0].get(0);
    println!("value: {}", value);

    let value: &str = rows2[0].get(0);
    println!("value: {}", value);

    Ok(())
}


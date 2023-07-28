use std::rc::Rc;
use std::env;
use tokio_postgres::{Client, NoTls, Error};

pub async fn connect() -> Result<impl Fn() -> Rc<Client>, Error> {
    //   to the database.

    let (client, connection) =
        tokio_postgres::connect(&env::var("DATABASE_URL").unwrap(), NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let client_rc = Rc::new(client);
   
    Ok(move || client_rc.clone())
}



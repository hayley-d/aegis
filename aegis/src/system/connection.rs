use dotenv;
use std::env;
use tokio_postgres::{Client, Error, NoTls};

pub async fn connect() -> Result<Client, Error> {
    dotenv::dotenv().ok();

    let host = get_env("DATABASE_HOST");
    let name = get_env("DATABASE_NAME");
    let user = get_env("DATABASE_USERNAME");
    let password = get_env("DATABASE_PASSWORD");
    let port = get_env("DATABASE_PORT");

    let connection_string = format!(
        "host={} user={} password={} dbname={} port={}",
        host, user, password, name, port
    );

    let (client, connection) = tokio_postgres::connect(&connection_string, NoTls).await?;

    tokio::spawn(async move {
        if let Err(error) = connection.await {
            eprintln!("Failed to connect to DB: {}", error);
        }
    });

    println!("Successfully connected to DB");

    Ok(client)
}

fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        eprintln!("Missing environment variable: {}", key);
        std::process::exit(1);
    })
}

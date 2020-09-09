extern crate diesel;
extern crate futile;
extern crate dotenv;

use futile::establish_connection;
use std::env;

fn main() {
    dotenv::dotenv().expect("Failed to find .env");

    let postgres_user = env::var("POSTGRES_USER").expect("Failed to find POSTGRES_USER in .env");
    let postgres_password = env::var("POSTGRES_PASSWORD").expect("Failed to find POSTGRES_PASSWORD in .env");
    let postgres_database = env::var("POSTGRES_DATABASE").expect("Failed to find POSTGRES_DATABASE in .env");
    let postgres_host = env::var("POSTGRES_HOST").expect("Failed to find POSTGRES_HOST in .env");
    let database_url = format!("postgres://{}:{}@{}/{}", postgres_user, postgres_password, postgres_host, postgres_database);

    let connection = establish_connection(&database_url[..]);
    print!("hello world");
}
extern crate diesel;
extern crate futile;
extern crate dotenv;

use std::env;
use self::futile::*;
use std::io::{stdin};

fn main() {
    dotenv::dotenv().expect("Failed to find .env");

    let postgres_user = env::var("POSTGRES_USER").expect("Failed to find POSTGRES_USER in .env");
    let postgres_password = env::var("POSTGRES_PASSWORD").expect("Failed to find POSTGRES_PASSWORD in .env");
    let postgres_database = env::var("POSTGRES_DATABASE").expect("Failed to find POSTGRES_DATABASE in .env");
    let postgres_host = env::var("POSTGRES_HOST").expect("Failed to find POSTGRES_HOST in .env");
    let database_url = format!("postgres://{}:{}@{}/{}", postgres_user, postgres_password, postgres_host, postgres_database);

    let connection = establish_connection(&database_url[..]);

    println!("Enter username:");
    let mut username = String::new();
    stdin().read_line(&mut username).unwrap();
    let username = &username[..(username.len() - 1)]; // Drop the newline character
    
    let user_exists = get_user(&connection, &username[..]);
    match user_exists {
        Some(user) => {
            println!("User '{}' exists", user.username);

            println!("Enter password:");
            let mut password = String::new();
            stdin().read_line(&mut password).unwrap();
            let password = &password[..(password.len() - 1)]; // Drop the newline character

            let login = login(&connection, username, password);
            match login {
                Some(user) => println!("Logged in user {}", user.username),
                None => println!("Wrong password")
            }
        },
        None => {
            println!("User '{}' does not exist", username);

            println!("Enter password:");
            let mut password = String::new();
            stdin().read_line(&mut password).unwrap();
            let password = &password[..(password.len() - 1)]; // Drop the newline character

            let new_user = create_user(&connection, &username[..], &password[..]);
            println!("New user: {} {} {}", new_user.id, new_user.username, new_user.hashed_password);
        }
    }
}
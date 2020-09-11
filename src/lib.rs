#[macro_use]
extern crate diesel;
extern crate bcrypt;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use diesel::pg::PgConnection;
use self::models::*;

pub fn establish_connection(database_url: &str) -> PgConnection {
    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_user(connection: &PgConnection, username: &str) -> Option<User> {
    let query = schema::users::dsl::users.filter(schema::users::dsl::username.eq(username)).limit(1);
    let result = query.get_result::<User>(connection)
                    .optional()
                    .expect("Unable to execute check_user_exists");
    println!("debug query {}", diesel::debug_query::<diesel::pg::Pg, _>(&query).to_string());
    result
}

pub fn create_user(connection: &PgConnection, username: &str, password: &str) -> User {
    // todo: add salt
    let hashed_password = bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("Unable to hash password");
    diesel::insert_into(schema::users::dsl::users)
    .values((schema::users::dsl::username.eq(username), schema::users::dsl::hashed_password.eq(hashed_password)))
    .get_result::<User>(connection).expect("unable to create user")
}

fn create_salt() -> vec![byte] {
    [0u8]
}

pub fn login(connection: &PgConnection, username: &str, password: &str) -> Option<User> {
    let user = get_user(connection, username);
    match user {
        None => None,
        Some(user) => {
            if bcrypt::verify(password, &user.hashed_password).expect("Unable to verify password") {
                Some(user)
            } else {
                None
            }
        }
    }
}

extern crate diesel;
extern crate futile;

use futile::establish_connection;
use std::io::{stdin, Read};

fn main() {
    let connection = establish_connection();
    print!("hello world");
}
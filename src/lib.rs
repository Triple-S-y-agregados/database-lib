#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

pub mod schema;
pub mod models;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn show_records() {
    use schema::records::dsl::*;

    let connection = establish_connection();

    let results = records
        .load::<models::Record>(&connection)
        .expect("Error loading records");

        println!("Displaying {} records", results.len());
        for post in results {
            println!("----------\n");
            println!("{}", post.id);
            println!("{}", post.timestamp);
            println!("{}", post.voltage);
            println!("----------\n");
        }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_connection_test() {
        establish_connection();
    }

    #[test]
    fn show_records_test() {
        show_records();
    }
}

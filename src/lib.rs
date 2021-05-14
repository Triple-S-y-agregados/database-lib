#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Record, NewRecord};

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_record(connection: &SqliteConnection, record_id: i32) -> Record {
    use self::schema::records::dsl::*;

    let record = records
        .find(record_id)
        .load::<Record>(connection)
        .expect("Record with id not found");

    return record[0].clone();
}

pub fn show_records() {
    use self::schema::records::dsl::*;

    let connection = establish_connection();
    let results = records
        .load::<models::Record>(&connection)
        .expect("Error loading records");

        println!("Displaying {} records", results.len());
        for post in results {
            println!("----------\n");
            println!("Id: {}", post.id);
            println!("Timestamp: {}", post.timestamp);
            println!("Voltage: {}", post.voltage);
            println!("----------\n");
        }
}

pub fn create_record<'a>(conn: &SqliteConnection, timestamp: &'a str, voltage: &'a i32) -> usize {
    use schema::records;

    let new_record = NewRecord {
        timestamp,
        voltage,
    };

    diesel::insert_into(records::table)
        .values(&new_record)
        .execute(conn)
        .expect("Error inserting new record.")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn database_connection_test() {
        establish_connection();
    }

    #[test]
    fn get_first_record_test() {
        let connection = establish_connection();
        let record = get_record(&connection, 1);

        println!("Id: {}, Timestamp: {}, Voltage: {}", record.id, record.timestamp, record.voltage);
    }

    #[test]
    fn show_records_test() {
        show_records();
    }

    #[test]
    fn insert_record_test() {
        let connection = establish_connection();
        create_record(&connection, "10/10/20", &5);
    }
}

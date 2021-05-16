#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod schema;
pub mod models;

use chrono;

use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use self::models::{Record, NewRecord};

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_record(record_id: i32) -> Record {
    use self::schema::records::dsl::*;

    let connection = establish_connection();

    let record = records
        .find(record_id)
        .load::<Record>(&connection)
        .expect("Record with id not found");

    return record[0].clone();
}

pub fn get_all_records() -> Vec<Record> {
    use self::schema::records::dsl::*;

    let connection = establish_connection();

    records.load::<Record>(&connection).expect("Failed to load records")
}

pub fn create_record<'a>(voltage: &'a i32) -> usize {
    use schema::records;

    let local = chrono::Local::now();
    let datetime = local.format("%Y-%m-%d %H:%M:%S").to_string();
    let timestamp = datetime.as_str();

    let new_record = NewRecord {
        timestamp,
        voltage,
    };

    let connection = establish_connection();

    diesel::insert_into(records::table)
        .values(&new_record)
        .execute(&connection)
        .expect("Error inserting new record.")
}

pub fn clean() {
    use schema::records;

    let connection = establish_connection();
    diesel::delete(records::table).execute(&connection);
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
        let record = get_record(1);

        println!("Id: {}, Timestamp: {}, Voltage: {}", record.id, record.timestamp, record.voltage);
    }

    #[test]
    fn show_records_test() {
        let results = get_all_records();

        println!("Displaying {} records", results.len());
        for post in results {
            println!("----------");
            println!("Id: {}", post.id);
            println!("Timestamp: {}", post.timestamp);
            println!("Voltage: {}", post.voltage);
            println!("----------");
        }
    }

    #[test]
    fn insert_record_test() {
        create_record(&5);
    }

    #[test]
    fn clean_database_test() {
        clean();
    }
}

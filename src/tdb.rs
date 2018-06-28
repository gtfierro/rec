extern crate rusqlite;
extern crate chrono;

use rusqlite::{Connection, Error};
use std::fmt;
use chrono::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct TDB {
    conn: Connection
}

impl TDB {
    pub fn new() -> Result<TDB, Error> {
        match Connection::open("rec.sqlite3") {
            Ok(conn) => {
                conn.execute("CREATE TABLE IF NOT EXISTS data (
                    id          INTEGER PRIMARY KEY AUTOINCREMENT,
                    collection  TEXT NOT NULL,
                    time        INTEGER NOT NULL,
                    value       REAL NOT NULL
                )", &[]).unwrap();
                Ok(TDB {conn: conn})
            },
            Err(error) => Err(error)
        }
    }

    pub fn add(&self, rec: Record) {
        self.conn.execute("INSERT INTO data (collection, time, value)
                VALUES (?1, ?2, ?3)",
                &[&rec.collection, &rec.time, &rec.value]).unwrap();
    }

    pub fn collections(&self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT collection FROM data;").unwrap();
        let data_iter = stmt.query_map(&[], |row| {row.get(0)}).unwrap();
        let mut collections = Vec::new();
        for record in data_iter {
            collections.push(record.unwrap());
        }
        collections
    }

    pub fn range(&self, collection: String, start: i64, end: i64) {
        let mut stmt = self.conn.prepare("SELECT collection, time, value FROM data WHERE collection = ?1 AND time >= ?2 AND time <= ?3").unwrap();
        let data_iter = stmt.query_map(&[&collection, &start, &end], |row| {
            Record {
                collection: row.get(0),
                time: row.get(1),
                value: row.get(2)
            }
        }).unwrap();
        for record in data_iter {
            println!("{}", record.unwrap());
        }
    }

    pub fn query(&self, query: String) {
        let mut stmt = self.conn.prepare(query.as_ref()).unwrap();
        let data_iter = stmt.query_map(&[], |row| {
            Record {
                collection: row.get(0),
                time: row.get(1),
                value: row.get(2)
            }
        }).unwrap();
        for record in data_iter {
            println!("{}", record.unwrap());
        }
    }

    pub fn all(&self, collection: String) {
        let mut stmt = self.conn.prepare("SELECT collection, time, value FROM data WHERE collection = ?1;").unwrap();
        let data_iter = stmt.query_map(&[&collection], |row| {
            Record {
                collection: row.get(0),
                time: row.get(1),
                value: row.get(2)
            }
        }).unwrap();
        for record in data_iter {
            println!("{}", record.unwrap());
        }
    }
}

#[derive(Debug)]
pub struct Record {
    pub collection: String,
    pub time: i64,
    pub value: f64
}

impl fmt::Display for Record {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let secs = self.time / 1e9 as i64;
        let nsecs = self.time  - (secs * 1e9 as i64);
        let formatted_time = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(secs, nsecs as u32), Utc);
        write!(f, "({},  {}, {})", self.collection, self.value, formatted_time.to_rfc2822())
    }
}

impl Record {
    pub fn is_empty(&self) -> bool {
        return self.collection == "" && self.time == 0 && self.value == 0 as f64;
    }
}

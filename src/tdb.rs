extern crate rusqlite;
extern crate chrono;

use rusqlite::types::ToSql;
use rusqlite::{Connection, Error, NO_PARAMS};
use std::{fmt, env};
use chrono::prelude::*;
use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct TDB {
    conn: Connection
}

impl TDB {
    pub fn new_with_location(location: String) -> Result<TDB, Error> {
        println!("Using database at {}", location);
        let conn = Connection::open(location)?;
        conn.execute("CREATE TABLE IF NOT EXISTS data (
            id          INTEGER PRIMARY KEY AUTOINCREMENT,
            collection  TEXT NOT NULL,
            time        INTEGER NOT NULL,
            value       REAL NOT NULL
            )",
        NO_PARAMS)?;
        Ok(TDB{conn: conn})
    }

    pub fn new() -> Result<TDB, Error> {
        //println!("Using database at {}/rec.sqlite3", env::current_dir().unwrap().display());
        let mut curdir = env::current_dir().unwrap();
        curdir.push("rec.sqlite3");
        TDB::new_with_location(curdir.into_os_string().into_string().unwrap())
    }

    pub fn add(&self, rec: Record) {
        self.conn.execute("INSERT INTO data (collection, time, value)
                VALUES (?1, ?2, ?3)",
                &[&rec.collection, &rec.time as &dyn ToSql, &rec.value]).unwrap();
    }

    pub fn collections(&self) -> Vec<String> {
        let mut stmt = self.conn.prepare("SELECT DISTINCT collection FROM data;").unwrap();
        let data_iter = stmt.query_map(NO_PARAMS, |row| {row.get(0)}).unwrap();
        let mut collections = Vec::new();
        for record in data_iter {
            collections.push(record.unwrap());
        }
        collections
    }

    pub fn range(&self, collection: String, start: i64, end: i64) {
        let mut stmt = self.conn.prepare("SELECT collection, time, value FROM data WHERE collection = ?1 AND time >= ?2 AND time <= ?3").unwrap();
        let data_iter = stmt.query_map(&[&collection, &start as &dyn ToSql, &end as &dyn ToSql], |row| {
            Ok(Record {
                collection: row.get(0).unwrap(),
                time: row.get(1).unwrap(),
                value: row.get(2).unwrap()
            })
        }).unwrap();
        for record in data_iter {
            println!("{}", record.unwrap());
        }
    }

    pub fn query(&self, query: String) {
        let mut stmt = self.conn.prepare(query.as_ref()).unwrap();
        let data_iter = stmt.query_map(NO_PARAMS, |row| {
            Ok(Record {
                collection: row.get(0).unwrap(),
                time: row.get(1).unwrap(),
                value: row.get(2).unwrap()
            })
        }).unwrap();
        for record in data_iter {
            println!("{}", record.unwrap());
        }
    }

    pub fn all(&self, collection: String) {
        let mut stmt = self.conn.prepare("SELECT collection, time, value FROM data WHERE collection = ?1;").unwrap();
        let data_iter = stmt.query_map(&[&collection], |row| {
            Ok(Record {
                collection: row.get(0).unwrap(),
                time: row.get(1).unwrap(),
                value: row.get(2).unwrap()
            })
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
        write!(f, "{},{},{}", self.collection, self.value, formatted_time.to_rfc3339())
    }
}

impl Record {
    pub fn is_empty(&self) -> bool {
        return self.collection == "" && self.time == 0 && self.value == 0 as f64;
    }
}

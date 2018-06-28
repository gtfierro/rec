extern crate clap;
extern crate chrono;
extern crate rusqlite;
extern crate time;
mod tdb;

use tdb::{TDB, Record};
use std::process;
use clap::{Arg, App, SubCommand, AppSettings};
use chrono::prelude::*;

fn main() {

    let db = TDB::new().unwrap();

    let app = App::new("Rec")
                        .version("0.1.0")
                        .author("Gabe Fierro")
                        .setting(AppSettings::SubcommandRequiredElseHelp)
                        .subcommand(SubCommand::with_name("add")
                            .usage("rec add <collection1> <value1> [<collection2> <value2> ...]")
                            .about("add data to collections")
                            .arg(Arg::with_name("rec")
                                .multiple(true))
                        )
                        .subcommand(SubCommand::with_name("tadd")
                            .usage("rec tadd <collection1> <value1> <time1> [<collection2> <value2> <value2> ...]")
                            .about("add data to collections with times")
                            .arg(Arg::with_name("rec")
                                .multiple(true))
                        )
                        .subcommand(SubCommand::with_name("all")
                            .usage("rec all <collection1> [<collection2> ...]")
                            .about("Get all data for the requested collections")
                            .arg(Arg::with_name("name")
                                .multiple(true))
                        )
                        .subcommand(SubCommand::with_name("list")
                            .about("list collections")
                            .usage("rec list")
                        )
                        .subcommand(SubCommand::with_name("query")
                            .about("query the data")
                            .arg(Arg::with_name("collection")
                                .index(1)
                                .required(true))
                        );
    let matches = app.get_matches();

    match matches.subcommand() {
        ("add", Some(_matches)) => {
                let mut args = _matches.values_of("rec").unwrap();
                let time = Local::now().timestamp_nanos();
                loop {
                    let r = Record {
                        collection: match args.next() {
                                        Some(val) => {
                                            val.to_string()
                                        },
                                        None => { break }
                                    },
                        value: match args.next() {
                            Some(val) => {
                                match val.parse() {
                                    Ok(_v) => _v,
                                    Err(err) => {
                                        println!("Invalid value {} ({})", val, err);
                                        process::exit(1)
                                    }
                                }
                            },
                            None => { break }
                        },
                        time: time
                    };
                    if !r.is_empty() {
                        db.add(r);
                    }
                }
        },
        ("tadd", Some(_matches)) => {
                let mut args = _matches.values_of("rec").unwrap();
                loop {
                    let r = Record {
                        collection: match args.next() {
                            Some(val) => {
                                val.to_string()
                            },
                            None => { break }
                        },
                        value: match args.next() {
                            Some(val) => {
                                match val.parse() {
                                    Ok(_v) => _v,
                                    Err(err) => {
                                        println!("Invalid value {} ({})", val, err);
                                        process::exit(1)
                                    }
                                }
                            },
                            None => { break }
                        },
                        time: match args.next() {
                            Some(val) => {
                                match val.parse::<DateTime<Local>>() {
                                    Ok(_time) => _time.timestamp_nanos(),
                                    Err(_error) => {
                                        println!("Invalid time {} ({})", val, _error);
                                        process::exit(1)
                                    }
                                }
                            },
                            None => { break }
                        }
                    };
                    if !r.is_empty() {
                        db.add(r);
                    }
                }
        },
        ("list", _) => {
            for name in db.collections() {
                println!("{}", name);
            }
        },
        ("all", Some(_matches)) => {
            let mut args = _matches.values_of("name").unwrap();
            loop {
                match args.next() {
                    Some(val) => {
                        db.all(val.to_string());
                    },
                    None => { break }
                }
            }
        },
        ("query", Some(_matches)) => {
            //let collection = matches.value_of("collection").unwrap().to_string();
            println!("query")
        },
        _ => {
        },
    };
}

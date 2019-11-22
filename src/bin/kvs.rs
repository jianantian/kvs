extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use kvs::KvStore;
use std::env;
use std::env::current_dir;
use std::io;
use std::process::exit;

fn main() -> Result<(), String> {
    let matches = App::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .setting(AppSettings::ArgRequiredElseHelp)
        .subcommand(
            SubCommand::with_name("set")
                .about("Set key value in storage")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .value_name("KEY")
                        .required(true),
                )
                .arg(
                    Arg::with_name("value")
                        .index(2)
                        .value_name("VALUE")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("get")
                .about("Get value for given key")
                .arg(
                    Arg::with_name("key")
                        .index(1)
                        .value_name("KEY")
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("rm").about("Remove given key").arg(
                Arg::with_name("key")
                    .index(1)
                    .value_name("KEY")
                    .required(true),
            ),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(matches)) => {
            let key = matches.value_of("key").expect("key argument missing");
            let value = matches.value_of("value").expect("value argument missing");
            let path = match current_dir() {
                io::Result::Ok(x) => x,
                io::Result::Err(why) => return Result::Err(why.to_string()),
            };
            let mut store = KvStore::open(&path)?;
            store.set(key.to_string(), value.to_string())?;
        }
        ("get", Some(matches)) => {
            let key = matches.value_of("key").expect("key argument missing");
            let path = match current_dir() {
                io::Result::Ok(x) => x,
                io::Result::Err(why) => return Result::Err(why.to_string()),
            };
            let mut store = KvStore::open(&path)?;
            if let Some(value) = store.get(key.to_string())? {
                println!("{}", value);
            } else {
                println!("Key not found");
            }
        }
        ("rm", Some(matches)) => {
            let key = matches.value_of("key").expect("key argument missing");
            let path = match current_dir() {
                io::Result::Ok(x) => x,
                io::Result::Err(why) => return Result::Err(why.to_string()),
            };
            let mut store = KvStore::open(&path)?;
            match store.remove(key.to_string()) {
                Ok(_) => {},
                Err(_) => {
                    println!("Key not found");
                    exit(1);
                }
            }
        }
        _ => unreachable!(),
    };
    Ok(())
}

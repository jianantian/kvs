extern crate clap;

use clap::{App, AppSettings, Arg, SubCommand};
use std::env;

fn main() {
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

    if let Some(ref command) = matches.subcommand_matches("get") {
        // let key = command.value_of("key").unwrap();
        // println!("kvs get {}", key);
        panic!("unimplemented")
    }

    if let Some(ref command) = matches.subcommand_matches("set") {
        // let key = command.value_of("key").unwrap();
        // let value = command.value_of("value").unwrap();
        // println!("kvs set {} {}", key, value);
        panic!("unimplemented")
    }

    if let Some(ref command) = matches.subcommand_matches("rm") {
        // let key = command.value_of("key").unwrap();
        // println!("kvs rm {}", key);
        panic!("unimplemented")
    }
}

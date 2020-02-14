use clap::{App, Arg};
use std::process::exit;

pub fn main() {
    let matches = App::new("kvs")
        .version(env!("CARGO_PKG_VERSION"))
        .subcommand(
            App::new("set")
                .about("Set the value of a string key to a string")
                .arg(Arg::with_name("KEY").required(true))
                .arg(Arg::with_name("VALUE").required(true)),
        )
        .subcommand(
            App::new("get")
                .about("Get the string value of a given string key")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .subcommand(
            App::new("rm")
                .about("Remove a given key")
                .arg(Arg::with_name("KEY").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        ("set", Some(_matches)) => eprintln!("unimplemented"),
        ("get", Some(_matches)) => eprintln!("unimplemented"),
        ("rm", Some(_matches)) => eprintln!("unimplemented"),
        _ => unreachable!(),
    }

    exit(1);
}

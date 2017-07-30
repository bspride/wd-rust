extern crate clap;

mod cli;

use clap::{App, Arg, SubCommand, AppSettings};
use cli::{add, warp};

fn main() {
    let matches = App::new("wd")
        .version("1.0")
        .about("Warp to directories")
        .setting(AppSettings::SubcommandsNegateReqs)
        .arg(
            Arg::with_name("point")
                .help("Warp point")
                .required(true)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds the current working directory to warp points")
                .arg(Arg::with_name("name").required(true)),
        )
        .get_matches();

    if matches.is_present("point") {
        warp::run(&matches);
    } else {
        match matches.subcommand() {
            ("add", Some(matches)) => add::run(matches),
            _ => {
                println!("{}", matches.usage());
            }
        }
    }
}

extern crate clap;

use clap::{App, Arg, SubCommand, AppSettings};

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
                .arg(Arg::with_name("point").required(true)),
        )
        .get_matches();

    if matches.is_present("point") {
        let point = matches.value_of("point").unwrap();
        println!("{}", &point);
    } else {
        match matches.subcommand() {
            ("add", Some(matches)) => {
                println!("Add was used!");
            }
            _ => {
                println!("{}", matches.usage());
            }
        }
    }
}

extern crate clap;

use clap::{App, Arg, SubCommand};

fn main() {
    let matches = App::new("wd")
        .version("1.0")
        .about("Warp to directories")
        .arg(
            Arg::with_name("point")
                .help("Warp point")
                .required(false)
                .index(1),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Adds the current working directory to warp points")
                .arg(Arg::with_name("point").required(false)),
        )
        .get_matches();

    let point = matches.value_of("point").unwrap_or("");
    println!("{}", &point);

    if let Some(matches) = matches.subcommand_matches("add") {
        println!("Made it mom!");
    }
}

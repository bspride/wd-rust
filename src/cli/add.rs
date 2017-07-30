use clap::ArgMatches;

pub fn run(m: &ArgMatches) {
    let path = m.value_of("point").unwrap();
    println!("{}", &path);
}

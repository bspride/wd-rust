extern crate preferences;
use self::preferences::{AppInfo, PreferencesMap, Preferences};

use clap::ArgMatches;
use std::env;

const APP_INFO: AppInfo = AppInfo {
    name: "wd",
    author: "Michael Clarke",
};

pub fn run(m: &ArgMatches) {
    let prefs_key = "warp/points";

    let mut faves: PreferencesMap<String> = PreferencesMap::new();
    let load_result = PreferencesMap::<String>::load(&APP_INFO, prefs_key);

    if load_result.is_ok() {
        println!("Loaded prefs");
        faves = load_result.unwrap();
    } else {
        // Create preferences file
        let initial_save_result = faves.save(&APP_INFO, prefs_key);
        println!("{:?}", initial_save_result);
    }


    let name = m.value_of("name").unwrap();
    let path = env::current_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap();

    faves.insert(name.into(), path.into());

    let save_result = faves.save(&APP_INFO, prefs_key);
    println!("{:?}", save_result);

    println!("{}", &name);
}

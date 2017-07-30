extern crate preferences;
use self::preferences::{AppInfo, PreferencesMap, Preferences};

use clap::ArgMatches;

const APP_INFO: AppInfo = AppInfo {
    name: "wd",
    author: "Michael Clarke",
};

pub fn run(m: &ArgMatches) {
    let prefs_key = "warp/points";
    let load_result = PreferencesMap::<String>::load(&APP_INFO, prefs_key);

    let warp_points: PreferencesMap<String> = load_result.unwrap();
    let point = m.value_of("point").unwrap();

    let destination = warp_points.get(point).unwrap();
    print!("{}", destination);
}

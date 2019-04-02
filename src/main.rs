mod color;
mod key;
mod settings;

use crate::settings::{Mode, Settings};
use clap::{App, Arg, SubCommand};
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub const DEFAULT_CONFIG_PATH: &'static str = "config.ron";

fn main() {
    let matches = App::new("Roccat Vulcan helper for Linux")
        .version("1.0")
        .author("Arno Dupont <catvert@hotmail.be>")
        .about("Roccat Vulcan [https://github.com/duncanthrax/roccat-vulcan] Helper")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .value_name("MODE [i]mpact/[w]ave/[o]ff")
                .help("Set and launch with this effect mode")
                .takes_value(true),
        )
        .arg(Arg::with_name("v").short("v").help("Be verbose."))
        .get_matches();

    let config_file = matches.value_of("config").unwrap_or(DEFAULT_CONFIG_PATH);

    let mut settings = Settings::from_ron(Path::new(config_file)).unwrap_or_else(|err| {
        eprintln!("{}", err);
        Settings::default()
    });

    if let Some(mode) = matches.value_of("mode") {
        settings.set_mode(Mode::from_str(mode).unwrap_or_else(|err| panic!(err)));
    }

    settings.save();

    let verbose = matches.is_present("v");
    settings.launch(verbose);
}

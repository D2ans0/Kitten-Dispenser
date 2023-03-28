use clap_v3::{Arg, App, ArgMatches};

pub fn get() -> ArgMatches {
    let matches = App::new("Kitten Dispenser")
        .version("0.1.0")
        .about("Fun-type Discord bot")
        .arg(Arg::with_name("config")
                 .short('c')
                 .long("config")
                 .takes_value(true)
                 .help("Path to config file"))
        .get_matches();

    return matches
}
use clap::{App, Arg};

pub struct LC3Args {
    pub path_to_assembled: String,
    pub debug_mode: bool,
}

impl LC3Args {
    pub fn parse() -> LC3Args {
        let matches = App::new("LC-3 VM")
            .version("1.0")
            .author("Joren Van Onder <joren@jvo.sh>")
            .about("Takes assembled LC-3 programs and runs them.")
            .arg(
                Arg::with_name("assembled file")
                    .short("f")
                    .long("--assembled-file")
                    .value_name("FILE")
                    .help("Path to assembled LC-3 machine code")
                    .required(true)
                    .takes_value(true),
            )
            .arg(
                Arg::with_name("debug mode")
                    .short("d")
                    .long("--debug")
                    .help("Print the internal state of the machine"),
            )
            .get_matches();

        LC3Args {
            path_to_assembled: matches.value_of("assembled file").unwrap().to_string(),
            debug_mode: matches.is_present("debug mode"),
        }
    }
}

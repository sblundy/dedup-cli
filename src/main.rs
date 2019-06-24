extern crate clap;

use clap::{App, Arg};

use std::io;
use std::path::Path;
use dedup_cli::dedup_with_params;

const FROM_STDIN:&str = "-";

fn main() {
    let args = App::new("dedup")
        .help("Deduplicates the input, writing non-duplicate lines as they appear.")
        .arg(Arg::with_name("INPUT")
            .help("File to be deduplicated.")
            .index(1)
            .validator(exists_or_dash))
        .arg(Arg::with_name("OUTPUT")
            .help("Output file.")
            .index(2)
            .long("output")).get_matches();

    let input_param = match args.value_of("INPUT") {
        Some(FROM_STDIN) => None,
        Some(file) => Some(file),
        None => None,
    };
    let output_param = args.value_of("OUTPUT");

    if let Err(err) = dedup_with_params(input_param, output_param, &mut io::stdin(), &mut io::stdout()) {
        eprintln!("{}", err);
        std::process::exit(1)
    }
}

fn exists_or_dash(v: String) -> Result<(), String> {
    return if v.as_str() == FROM_STDIN || Path::new(&v).exists() {
        Ok(())
    } else {
        Err(String::from("file does not exist"))
    };
}
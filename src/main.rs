extern crate clap;

use clap::{App, Arg};

use std::io;
use std::path::Path;
use dedup_cli::dedup_with_params;

fn main() {
    let args = App::new("dedup")
        .help("Deduplicates the input, writing non-duplicate lines as they appear.")
        .arg(Arg::with_name("INPUT")
            .help("File to be deduplicated. '-' for stdin")
            .index(1)
            .required(true)
            .validator(exists_or_dash)
            .default_value(dedup_cli::FROM_STDIN))
        .arg(Arg::with_name("OUTPUT")
            .help("Output file. '-' for stdout")
            .index(2)
            .required(true)
            .default_value(dedup_cli::TO_STDOUT)).get_matches();

    let input_param = args.value_of("INPUT").unwrap();
    let output_param = args.value_of("OUTPUT").unwrap();

    if let Err(err) = dedup_with_params(input_param, output_param, &mut io::stdin(), &mut io::stdout()) {
        eprintln!("{}", err);
        std::process::exit(1)
    }
}

fn exists_or_dash(v: String) -> Result<(), String> {
    return if v.as_str() == dedup_cli::FROM_STDIN || Path::new(&v).exists() {
        Ok(())
    } else {
        Err(String::from("file does not exist"))
    };
}
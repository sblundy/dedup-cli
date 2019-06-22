use std::io;
use std::fs::File;
use crate::deduplication::dedup;

pub const FROM_STDIN: &str = "-";
pub const TO_STDOUT: &str = "-";

pub fn dedup_with_params(input: &str, output: &str, stdin: &mut io::Read, stdout: &mut io::Write) -> io::Result<()> {
    return match (input, output) {
        (FROM_STDIN, TO_STDOUT) => dedup(stdin, stdout),
        (input_file_name, TO_STDOUT) => dedup_file_to(input_file_name, stdout),
        (FROM_STDIN, output_file_name) => dedup_to_file(stdin, output_file_name),
        (input_file_name, output_file_name) =>
            dedup_file_to_file(input_file_name, output_file_name),
    };
}

pub fn dedup_file_to(file_name: &str, output: &mut io::Write) -> io::Result<()> {
    return match File::open(file_name) {
        Ok(mut input_file) => dedup(&mut input_file, output),
        Err(e) => Err(e)
    };
}

pub fn dedup_to_file(input: &mut io::Read, file_name: &str) -> io::Result<()> {
    return match File::create(file_name) {
        Ok(mut output_file) => dedup(input, &mut output_file),
        Err(e) => Err(e)
    };
}

pub fn dedup_file_to_file(input_file_name: &str, output_file_name: &str) -> io::Result<()> {
    return match (File::open(input_file_name), File::create(output_file_name)) {
        (Ok(mut input_file), Ok(mut output_file)) =>
            dedup(&mut input_file, &mut output_file),
        (Err(e), _) => Err(e),
        (_, Err(e)) => Err(e)
    };
}

mod deduplication;
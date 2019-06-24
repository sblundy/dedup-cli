use std::io;
use std::fs::File;
use crate::deduplication::dedup;

pub fn dedup_with_params(input: Option<&str>, output: Option<&str>, stdin: &mut io::Read, stdout: &mut io::Write) -> io::Result<()> {
    match (input, output) {
        (None, None) => dedup(stdin, stdout),
        (Some(input_file_name), None) => dedup_file_to(input_file_name, stdout),
        (None, Some(output_file_name)) => dedup_to_file(stdin, output_file_name),
        (Some(input_file_name), Some(output_file_name)) =>
            dedup_file_to_file(input_file_name, output_file_name),
    }
}

pub fn dedup_file_to(file_name: &str, output: &mut io::Write) -> io::Result<()> {
    match File::open(file_name) {
        Ok(mut input_file) => dedup(&mut input_file, output),
        Err(e) => Err(e)
    }
}

pub fn dedup_to_file(input: &mut io::Read, file_name: &str) -> io::Result<()> {
    match File::create(file_name) {
        Ok(mut output_file) => dedup(input, &mut output_file),
        Err(e) => Err(e)
    }
}

pub fn dedup_file_to_file(input_file_name: &str, output_file_name: &str) -> io::Result<()> {
    match (File::open(input_file_name), File::create(output_file_name)) {
        (Ok(mut input_file), Ok(mut output_file)) =>
            dedup(&mut input_file, &mut output_file),
        (Err(e), _) | (_, Err(e)) => Err(e)
    }
}

mod deduplication;
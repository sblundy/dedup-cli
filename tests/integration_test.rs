extern crate assert_fs;
extern crate predicates;

use std::io;
use assert_fs::prelude::*;
use assert_fs::NamedTempFile;
use predicates::prelude::*;

use dedup_cli::dedup_with_params;

#[test]
fn deduplicates_file_with_no_duplicates() {
    let file = data::all_duplicates_file();
    let input = file.path().to_str();
    assert!(input.is_some());

    let mut dummy_stdout = Vec::new();

    let out = dedup_with_params(input, None,
                                &mut io::stdin(), &mut dummy_stdout);

    out.expect("error in dedup");
    data::assert_expected(dummy_stdout, data::ALL_DUPLICATES_EXPECTED);
}

#[test]
fn copies_file_with_no_duplicates_to_file() {
    let file = data::no_duplicates_file();
    let target = NamedTempFile::new("no_duplicates.output.txt").unwrap();
    let input = file.path().to_str();
    let output = target.path().to_str();
    assert!(input.is_some());
    assert!(output.is_some());

    let out = dedup_with_params(input, output,
                                &mut io::stdin(), &mut io::stdout());

    out.expect("error in dedup");
    target.assert(data::NO_DUPLICATES_EXPECTED);
}

#[test]
fn deduplicates_stream_to_file() {
    let target = NamedTempFile::new("no_duplicates.output.txt").unwrap();
    let output = target.path().to_str();
    assert!(output.is_some());

    let mut dummy_stdin = include_str!("data/all_duplicates.txt");

    let out = dedup_with_params(None, output,
                                &mut dummy_stdin.as_bytes(), &mut io::stdout());

    out.expect("error in dedup");
    target.assert(data::ALL_DUPLICATES_EXPECTED);
}

#[test]
fn error_when_input_file_not_exist() {
    let file = NamedTempFile::new("does-not-exist.txt").unwrap();
    let input = file.path().to_str();

    assert!(!file.path().exists());
    assert!(input.is_some());

    let mut dummy_stdout = Vec::new();

    let out = dedup_with_params(input, None,
                                &mut io::stdin(), &mut dummy_stdout);

    out.expect_err("error expected in dedup");
    assert_eq!(dummy_stdout.len(), 0);
}

mod data;
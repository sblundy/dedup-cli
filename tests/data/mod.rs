use assert_fs::prelude::*;
use assert_fs::NamedTempFile;

pub const NO_DUPLICATES_EXPECTED: &str = include_str!("no_duplicates_expected.txt");
pub const ALL_DUPLICATES_EXPECTED: &str = include_str!("all_duplicates.expected.txt");

pub fn no_duplicates_file() -> NamedTempFile {
    return test_file("no_duplicates.txt", include_str!("no_duplicates.txt"));
}

pub fn all_duplicates_file() -> NamedTempFile {
    return test_file("all_duplicates.txt", include_str!("all_duplicates.txt"));
}

pub fn test_file(name: &str, contents: &str) -> NamedTempFile {
    let f = NamedTempFile::new(name).unwrap();
    f.write_str(contents).expect("write of contents failed");
    return f;
}

pub fn assert_expected(actual: Vec<u8>, expected: &str) {
    assert_eq!(String::from_utf8(actual).unwrap(), String::from(expected));
}

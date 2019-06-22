use std::io;
use std::io::{BufReader, BufRead, Read, Write};
use std::collections::HashSet;

fn main() {
    match dedup(&mut io::stdin(), &mut io::stdout()) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("Problem parsing arguments: {}", err);
            std::process::exit(1)
        }
    }
}

fn dedup(input: &mut Read, output: &mut Write) -> io::Result<()> {
    let mut bf = BufReader::new(input);
    let mut seen = HashSet::new();
    loop {
        let mut buf = String::new();
        let out = bf.read_line(&mut buf);
        let b = Box::new(buf);
        match out {
            Ok(0) => { return Ok(()); }
            Ok(_) => if !seen.contains(&b) {
                match output.write_all(b.as_bytes()) {
                    Ok(_) => {
                        seen.insert(b);
                    }
                    Err(err) => { return Err(err); }
                }
            },
            Err(err) => { return Err(err); }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        let mut input = "".as_bytes();
        let mut output = Vec::new();

        let out = dedup(&mut input, &mut output);

        assert!(out.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "")
    }

    #[test]
    fn test_single_line() {
        let mut input = "test".as_bytes();
        let mut output = Vec::new();

        let out = dedup(&mut input, &mut output);

        assert!(out.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "test")
    }

    #[test]
    fn test_two_different_lines() {
        let mut input = "test\ntest2".as_bytes();
        let mut output = Vec::new();

        let out = dedup(&mut input, &mut output);

        assert!(out.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "test\ntest2")
    }

    #[test]
    fn test_two_identical_lines() {
        let mut input = "test\ntest\n".as_bytes();
        let mut output = Vec::new();

        let out = dedup(&mut input, &mut output);

        assert!(out.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "test\n")
    }

    #[test]
    fn test_two_separated_identical_lines() {
        let mut input = "test\ntest1\ntest\n".as_bytes();
        let mut output = Vec::new();

        let out = dedup(&mut input, &mut output);

        assert!(out.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "test\ntest1\n")
    }
}
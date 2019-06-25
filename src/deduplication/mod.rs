use std::collections::HashSet;
use std::io;
use std::io::{BufReader, Read, Write};

pub fn dedup(input: &mut Read, output: &mut Write) -> io::Result<()> {
    let bf = BufReader::new(input);
    let mut split_itr = bf.bytes();
    let mut seen = HashSet::new();
    loop {
        let mut buf = Vec::new();
        let out = read_to_separator_or_end(&mut split_itr, &mut buf);

        match out {
            Ok((0, _)) => return Ok(()),
            Ok((_, sep)) => {
                if !seen.contains(&buf) {
                    match output.write_all(&buf) {
                        Ok(_) => { seen.insert(buf); }
                        Err(err) => return Err(err),
                    }

                    if let Some(s) = sep {
                        let b = [s];
                        if let Err(err) = output.write(&b) {
                            return Err(err);
                        }
                    }
                }
            }
            Err(err) => return Err(err),
        }
    }
}

fn read_to_separator_or_end(
    bf: &mut io::Bytes<BufReader<&mut Read>>,
    buf: &mut Vec<u8>,
) -> io::Result<(usize, Option<u8>)> {
    let mut num = 0;
    loop {
        match bf.next() {
            None => return Ok((num, None)),
            Some(Ok(next)) if next == b'\n' => return Ok((num + 1, Some(b'\n'))),
            Some(Ok(next)) => {
                buf.push(next);
                num += 1;
            }
            Some(Err(err)) => return Err(err),
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

    #[test]
    fn ignores_separator() {
        let mut input = "test\ntest".as_bytes();
        let mut output = Vec::new();

        let out = dedup(&mut input, &mut output);

        assert!(out.is_ok());
        assert_eq!(String::from_utf8(output).unwrap(), "test\n")
    }
}

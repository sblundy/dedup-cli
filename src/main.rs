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
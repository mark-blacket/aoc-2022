use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn day6(size: usize) -> io::Result<(usize, String)> {
    let mut s = String::new();
    let mut f = BufReader::new(File::open("data/6.txt")?);
    f.read_line(&mut s)?;

    for start in 0..s.len()-size { 
        let mut found = false;
        s[start..start+size-1].chars().enumerate().for_each(|(i, c)| {
            found |= s[start+i+1..start+size].contains(c);
        });
        if !found {
            return Ok((start + size, s[start..start+size].to_owned()));
        }
    }
    Err(io::Error::new(io::ErrorKind::Other, "answer not found"))
}

pub fn run() -> io::Result<()> {
    println!("6-1: {:?}", day6(4)?);
    println!("6-2: {:?}", day6(14)?);
    Ok(())
}

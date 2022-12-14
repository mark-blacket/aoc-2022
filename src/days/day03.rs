use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn task1() -> io::Result<u32> {
    let f = BufReader::new(File::open("data/3.txt")?);
    let mut total = 0;
    
    for line in f.lines() {
        let x = line?;
        let (l, r) = x.split_at(x.len() / 2);
        if let Some(c) = l.chars().find(|&c| r.contains(c)) {
            total += if c.is_uppercase() { 26 } else { 0 };
            total += c.to_digit(36).unwrap() - 9;
        }
    }
    Ok(total)
}

fn task2() -> io::Result<u32> {
    let f = BufReader::new(File::open("data/3.txt")?);
    let mut total = 0;
    
    let mut buf = [String::new(), String::new(), String::new()];
    for (i, line) in f.lines().enumerate() {
        let x = line?;
        let i = i % 3;
        buf[i] = x;
        if i == 2 {
            let c = buf[0].chars().find(|&c| buf[1].contains(c) && buf[2].contains(c)).unwrap();
            total += if c.is_uppercase() { 26 } else { 0 };
            total += c.to_digit(36).unwrap() - 9;
        };
    }
    Ok(total)
}

pub fn run() -> io::Result<()> {
    println!("3-1: {}", task1()?);
    println!("3-2: {}", task2()?);
    Ok(())
}

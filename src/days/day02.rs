use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/2.txt")?);
    let pos1 = ["", "B X", "C Y", "A Z", "A X", "B Y", "C Z", "C X", "A Y", "B Z"];
    let pos2 = ["", "B X", "C X", "A X", "A Y", "B Y", "C Y", "C Z", "A Z" ,"B Z"];
    let mut total1 = 0;
    let mut total2 = 0;

    for line in f.lines() {
        let x = line?;
        total1 += pos1.iter().position(|&p| x.eq(p)).unwrap();
        total2 += pos2.iter().position(|&p| x.eq(p)).unwrap();
    }
    
    println!("2-1: {}", total1);
    println!("2-2: {}", total2);
    Ok(())
}


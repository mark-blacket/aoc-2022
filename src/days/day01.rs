use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/1.txt")?);
    let mut totals = Vec::new();
    let mut running_sum = 0;

    for line in f.lines() {
        let x = line?;
        if x.is_empty() {
            totals.push(running_sum);
            running_sum = 0;
        } else {
            running_sum += x.parse::<u64>().unwrap();
        }
    }

    totals.sort_unstable_by(|a ,b| b.cmp(a));
    println!("1-1: {}", totals[0]);
    println!("1-2: {}", totals.iter().take(3).sum::<u64>());
    Ok(())
}

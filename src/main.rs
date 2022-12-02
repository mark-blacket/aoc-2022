use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn day1() -> io::Result<()> {
    let f = File::open("data/1.txt")?;
    let fr = BufReader::new(f);
    let mut totals = Vec::new();
    let mut running_sum = 0u64;

    for line in fr.lines() {
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

fn main() -> io::Result<()> {
    let mut input = String::new();
    print!("{}", "Enter day number: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    match input.trim_end().parse().unwrap() {
        1 => day1(),
        _ => {
            println!("Incorrect day number: {}", input.as_str());
            Ok(())
        }
    }
}


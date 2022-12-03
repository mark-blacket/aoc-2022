use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn day1() -> io::Result<()> {
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

fn day2() -> io::Result<()> {
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

fn day3_1() -> io::Result<()> {
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

    println!("3-1: {}", total);
    Ok(())
}

fn day3_2() -> io::Result<()> {
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

    println!("3-2: {}", total);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    print!("{}", "Enter day number: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    match input.trim_end().parse().unwrap() {
        1 => day1(),
        2 => day2(),
        3 => { day3_1()?; day3_2() },
        _ => {
            println!("Incorrect day number: {}", input.as_str());
            Ok(())
        }
    }
}


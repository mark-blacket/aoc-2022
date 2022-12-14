use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/8.txt")?);
    let map = f.lines().map(|l| {
        l.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let mut visible = map.len() * 2 + map[0].len() * 2 - 4;
    let mut high_score = 0;

    for (i, row) in map[1..map.len() - 1].iter().enumerate() {
        for (j, val) in row[1..row.len() - 1].iter().enumerate() {
            let col = map.iter().map(|r| r[j + 1]).collect::<Vec<_>>();
            let slices = [&row[..=j], &row[j + 2..], &col[..=i], &col[i + 2..]];

            if slices.iter().any(|it| val > it.iter().max().unwrap()) {
                visible += 1;
            }

            let score = slices.iter().enumerate().map(|(i, x)| {
                let s = if i % 2 == 0 { 
                    x.iter().rev().take_while(|&x| x < val).count()
                } else {
                    x.iter().take_while(|&x| x < val).count()
                };
                if s != x.len() { s + 1 } else { s }
            }).product::<usize>();
            
            if score > high_score {
                high_score = score;
            }
        }
    }

    println!("8-1: {}", visible);
    println!("8-2: {}", high_score);
    Ok(())
}

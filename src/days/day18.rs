use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn count_bits(mut n: usize) -> usize {
    let mut count = 0;
    while n != 0 {
        n &= n - 1;
        count += 1;
    }
    count
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/18.txt")?);
    let mut coords = vec![];
    let mut max_coords = vec![0, 0, 0];

    for line in f.lines() {
        let line = line?.split(',').map(|x| {
            x.parse::<usize>().unwrap()
        }).collect::<Vec<_>>();
        max_coords.iter_mut().enumerate().for_each(|(i, x)| {
            if *x < line[i] { *x = line[i] }
        });
        coords.push((line[0], line[1], line[2]));
    }

    let mut area = 0;
    let mut shape = vec![vec![0; max_coords[1] + 2];max_coords[0] + 2];
    let empty_row = shape[0].clone();
    coords.iter_mut().for_each(|c| shape[c.0][c.1] |= 1 << c.2);
    for (i, row) in shape.iter().enumerate() {
        let upper_row = if i == 0 {
            &empty_row
        } else { 
            &shape[i - 1]
        };
        for (j, cell) in row.iter().enumerate() {
            let upper = upper_row[j];
            let left = if j == 0 { 0 } else { row[j - 1] };
            area += count_bits(upper ^ cell) + count_bits(left ^ cell);
            area += count_bits(cell ^ (cell << 1));
        }
    }

    println!("18-1: {}", area);
    Ok(())
}

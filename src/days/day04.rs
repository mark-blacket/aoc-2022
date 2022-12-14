use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/4.txt")?);
    let mut total1 = 0;
    let mut total2 = 0;

    for line in f.lines() {
        let v: Vec<u64> = line?.split(&['-', ','][..]).map(|x| x.parse().unwrap()).collect();
        if (v[0] >= v[2] && v[1] <= v[3]) || (v[0] <= v[2] && v[1] >= v[3]) {
            total1 += 1;
        }
        if !(v[0] > v[3] || v[2] > v[1]) {
            total2 += 1;
        }
    }

    println!("4-1: {}", total1);
    println!("4-2: {}", total2);
    Ok(())
}

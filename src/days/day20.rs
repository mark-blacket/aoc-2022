use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::VecDeque;

pub fn mix(mut v: VecDeque<(usize, i64)>, times: usize) -> i64 {
    for _ in 0..times {
        for i in 0..v.len() {
            let idx = v.iter().position(|(j, _)| (i == *j)).unwrap();
            v.rotate_left(idx);
            let (j, val) = v.pop_front().unwrap();
            v.rotate_left(val.rem_euclid(v.len() as i64) as usize);
            v.push_back((j, val));
        }
    }
    let idx = v.iter().position(|(_, v)| *v == 0).unwrap();
    [1000, 2000, 3000].iter().map(|x| v[(x + idx) % v.len()].1).sum()
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/20.txt")?);
    let mut buffer = VecDeque::new();
    for (i, line) in f.lines().enumerate() {
        let n = line?.parse::<i64>().unwrap();
        buffer.push_back((i, n));
    }

    let multed = buffer.iter().map(|(i, x)| (*i, x * 811589153)).collect::<VecDeque<_>>();
    println!("20-1: {}", mix(buffer, 1));
    println!("20-2: {}", mix(multed, 10));
    Ok(())
}

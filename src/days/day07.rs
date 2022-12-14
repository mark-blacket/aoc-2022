use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/7.txt")?);
    let mut sizes = HashMap::new();
    let mut path = Vec::new();

    for line in f.lines() {
        let line = line?;
        let v: Vec<&str> = line.split(' ').collect();
        match v[0] {
            "$"   => if v[1] == "cd" {
                match v[2] {
                    ".." => { path.pop(); },
                    x    => { path.push(x.to_owned()); },
                }
            },
            "dir" => (),
            x     => {
                let n = x.parse::<u64>().unwrap();
                for i in 0..path.len() {
                    let dir = path[..=i].join(" ");
                    let size = match sizes.get(&dir) {
                        Some(x) => x + n,
                        None    => n
                    };
                    sizes.insert(dir, size);
                }
            }
        }
    }

    let s: u64 = sizes.values().filter(|&&x| x <= 100000).sum();
    let needed = 30000000 - (70000000 - sizes["/"]);
    let min = sizes.values().filter(|&&x| x >= needed).min().unwrap();
    println!("7-1: {}", s);
    println!("7-2: {}", min);
    Ok(())
}

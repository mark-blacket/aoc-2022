use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn place_sand(map: &mut Vec<Vec<bool>>, limit: usize) -> bool {
    let mut pos = Some((500, 0));
    while let Some((x, y)) = pos {
        if y >= limit { return false; }
        pos = match map[y + 1][x - 1..=x + 1] {
            [_, false, _] => Some((x, y + 1)), 
            [false, _, _] => Some((x - 1, y + 1)),
            [_, _, false] => Some((x + 1, y + 1)),
            _             => {
                map[y][x] = true;
                None
            }
        }
    }
    true
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/14.txt")?);
    let mut map = vec![vec![false; 1000]; 200];
    let mut limit = 0;

    for line in f.lines() {
        let line = line?;
        let s = line.split("->").map(|x| {
            let (l, r) = x.trim().split_once(',').unwrap();
            (l.parse::<usize>().unwrap(), r.parse::<usize>().unwrap())
        });
        for ((x1, y1), (x2, y2)) in s.clone().zip(s.skip(1)) {
            limit = [y1, y2, limit].into_iter().max().unwrap();
            if y1 == y2 {
                map[y1][x1.min(x2)..=x1.max(x2)].iter_mut().for_each(|x| *x = true);
            } else {
                map[y1.min(y2)..=y1.max(y2)].iter_mut().for_each(|y| (*y)[x1] = true);
            }
        }
    }

    let mut count = 0;
    while place_sand(&mut map, limit) {
        count += 1
    }
    println!("14-1: {}", count);

    limit += 2;
    map[limit] = vec![true; 1000];
    while !map[0][500] {
        place_sand(&mut map, limit);
        count += 1;
    }
    println!("14-2: {}", count);
    Ok(())
}

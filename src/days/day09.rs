use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;

#[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
struct Point { x: i64, y: i64 }

fn task(points: usize) -> io::Result<usize> {
    let f = BufReader::new(File::open("data/9.txt")?);
    let mut p = vec![Point { x: 0, y: 0 }; points];
    let mut pos = HashSet::new();

    for line in f.lines() {
        let line = line?;
        let (dir, n) = line.split_once(' ').unwrap();
        let n = n.parse::<i64>().unwrap();
        let dir = match dir {
            "L" => | p: &mut Point | p.x -= 1,
            "R" => | p: &mut Point | p.x += 1,
            "D" => | p: &mut Point | p.y -= 1,
            "U" => | p: &mut Point | p.y += 1,
            _   => panic!()
        };

        for _ in 0..n {
            dir(&mut p[0]);
            for i in 1..points {
                let (x_diff, y_diff) = (p[i - 1].x - p[i].x, p[i - 1].y - p[i].y);
                match (x_diff.abs(), y_diff.abs()) {
                    (2, 2) => { p[i].x += x_diff.signum(); p[i].y += y_diff.signum() },
                    (2, _) => { p[i].x += x_diff.signum(); p[i].y += y_diff; },
                    (_, 2) => { p[i].x += x_diff; p[i].y += y_diff.signum(); },
                    (_, _) => ()
                };
            }
            pos.insert(p[points - 1]);
        }
    }
    Ok(pos.len())
}

pub fn run() -> io::Result<()> {
    println!("9-1: {}", task(2)?);
    println!("9-2: {}", task(10)?);
    Ok(())
}

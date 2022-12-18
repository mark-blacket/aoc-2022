use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;

struct Sensor { x: i64, y: i64, dist: i64 }

fn parse_pair(x: &str, y: &str) -> (i64, i64) {
    (
        x[2..].trim_end_matches(',').parse().unwrap(),
        y[2..].trim_end_matches(':').parse().unwrap()
    )
}

fn combine_ranges(v: &mut Vec<(i64, i64)>) {
    let mut i = 0;
    v.sort_by(|a, b| a.0.cmp(&b.0));
    while i < v.len() - 1 {
        match (v[i].1 >= v[i + 1].0, v[i].1 >= v[i + 1].1) {
            (false, false) => i += 1,
            (_    , true ) => { v.remove(i + 1); }
            (true , false) => {
                v[i].1 = v[i + 1].1;
                v.remove(i + 1);
            }
        }
    }
}

fn ranges_in_row(n: i64, sensors: &Vec<Sensor>) -> Vec<(i64, i64)> {
    let mut ranges = vec![];
    for s in sensors {
        if s.y <= n && s.y + s.dist > n {
            let d = s.y + s.dist - n;
            ranges.push((s.x - d, s.x + d));
        } else if s.y > n && s.y - s.dist <= n {
            let d = n - (s.y - s.dist);
            ranges.push((s.x - d, s.x + d));
        }
    }
    combine_ranges(&mut ranges);
    ranges
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/15.txt")?);
    let mut sensors = vec![];
    let mut beacons = HashSet::new();

    for line in f.lines() {
        let line = line?;
        let v = line.split(' ').collect::<Vec<_>>();
        let (sx, sy) = parse_pair(v[2], v[3]);
        let (bx, by) = parse_pair(v[8], v[9]);
        let dist = (sx.abs_diff(bx) + sy.abs_diff(by)) as i64;
        sensors.push(Sensor { x: sx, y: sy, dist });
        beacons.insert((bx, by));
    }

    {
        let ranges = ranges_in_row(2000000, &sensors);
        let b = beacons.iter().filter(|(_, b)| *b == 2000000).count() as u64;
        let count = ranges.iter().map(|(a, b)| a.abs_diff(*b + 1)).sum::<u64>() - b;
        println!("15-1: {}", count);
    }

    for row in 0..=4000000 {
        let ranges = ranges_in_row(row, &sensors);
        if ranges.len() > 1 && ranges[0].0 <= 0 && ranges[ranges.len() - 1].1 >= 4000000 {
            println!("15-2: {}", (ranges[0].1 + 1) * 4000000 + row);
            break
        }
    }

    Ok(())
}

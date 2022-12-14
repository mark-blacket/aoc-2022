use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashSet;

struct Point { h: u32, visited: bool, dist: u64, prev: Option<(usize, usize)> }

impl Point {
    fn new(h: u32, start: bool) -> Self {
        Point { h, visited: false, dist: if start { 0 } else { u64::MAX }, prev: None }
    }
}

fn neighborinos(map: &Vec<Vec<Point>>, pos: (usize, usize)) -> Vec<(usize, usize)> {
    [
        if pos.0 > 0                { Some((pos.0 - 1, pos.1)) } else { None },
        if pos.0 < map.len() - 1    { Some((pos.0 + 1, pos.1)) } else { None },
        if pos.1 > 0                { Some((pos.0, pos.1 - 1)) } else { None },
        if pos.1 < map[0].len() - 1 { Some((pos.0, pos.1 + 1)) } else { None }
    ].iter().filter(|x| x.is_some()).map(|x| x.unwrap()).filter(|&x| { 
        map[pos.0][pos.1].h <= map[x.0][x.1].h + 1 && !map[x.0][x.1].visited
    }).collect::<Vec<_>>()
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/12.txt")?);
    let (mut start, mut end) = ((0, 0), (0, 0));
    let mut map = f.lines().enumerate().map(|(i, line)| {
        line.unwrap().chars().enumerate().map(|(j, c)| match c {
            'S' => { start = (i, j); Point::new(1, false) },
            'E' => { end = (i, j);   Point::new(26, true) },
            x   => Point::new(x.to_digit(36).unwrap() - 9, false)
        }).collect::<Vec<_>>()
    }).collect::<Vec<_>>();

    let mut a_state = 0;
    let mut set = HashSet::new();
    set.insert(end);
    'outer: while !set.is_empty() {
        let mut tmp = HashSet::new();
        for pos in set.iter() {
            map[pos.0][pos.1].visited = true;
            if a_state == 0 && map[pos.0][pos.1].h == 1 {
                a_state = map[pos.0][pos.1].dist;
            }
            if *pos == start { break 'outer; }
            for n in neighborinos(&map, *pos) {
                let ndist = map[pos.0][pos.1].dist + 1;
                if ndist < map[n.0][n.1].dist {
                    map[n.0][n.1].dist = ndist;
                    map[n.0][n.1].prev = Some(*pos);
                }
                tmp.insert(n);
            }
        }
        set = tmp;
    }

    println!("12-1: {}", map[start.0][start.1].dist);
    println!("12-2: {}", a_state);
    Ok(())
}

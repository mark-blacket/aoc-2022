use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::{HashSet, HashMap, VecDeque};

type Steps = HashMap<(i64, i64), Vec<(i64, i64)>>;
const DIRS: [(i64, i64); 8] = [
    (-1, -1), (-1,  0), (-1,  1), ( 0, -1),
    ( 1, -1), ( 1,  0), ( 1 , 1), ( 0,  1)
];

fn propose(pos: &HashSet<(i64, i64)>, dirs: &VecDeque<[usize; 3]>) -> Steps {
    let mut steps = Steps::new();
    for (x, y) in pos {
        let empty = DIRS.iter().map(|(i, j)| !pos.contains(&(x + i, y + j))).collect::<Vec<_>>();
        if empty.iter().all(|x| *x) { continue }
        for dir in dirs {
            if dir.iter().all(|d| empty[*d]) {
                steps.entry((x + DIRS[dir[1]].0, y + DIRS[dir[1]].1))
                    .and_modify(|v| v.push((*x, *y))).or_insert(vec![(*x, *y)]);
                break
            }
        }
    }
    steps
}

fn empty_fields(pos: &HashSet<(i64, i64)>) -> i64 {
    let (mut minx, mut maxx) = (i64::MAX, i64::MIN);
    let (mut miny, mut maxy) = (i64::MAX, i64::MIN);
    pos.iter().for_each(|(x, y)| {
        if x < &minx { minx = *x }
        if x > &maxx { maxx = *x }
        if y < &miny { miny = *y }
        if y > &maxy { maxy = *y }
    });
    (maxx - minx + 1) * (maxy - miny + 1) - pos.len() as i64
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/23.txt")?);
    let mut pos = HashSet::new();
    let mut dirs = VecDeque::from([
        [0, 3, 4], // N
        [2, 7, 6], // S
        [0, 1, 2], // W
        [4, 5, 6], // E
    ]);

    for (y, line) in f.lines().enumerate() {
        line?.chars().enumerate().filter(|(_, c)| *c == '#')
            .for_each(|(x, _)| { pos.insert((x as i64, y as i64)); })
    }

    let mut i = 0;
    loop {
        i += 1;
        let steps = propose(&pos, &dirs);
        if steps.is_empty() { break }
        steps.iter().filter(|(_, v)| v.len() == 1).for_each(|(k, v)| {
            pos.remove(&v[0]);
            pos.insert(*k);
        });
        dirs.rotate_left(1);
        if i == 10 { println!("23-1: {}", empty_fields(&pos)) };
    }

    println!("23-2: {}", i);
    Ok(())
}

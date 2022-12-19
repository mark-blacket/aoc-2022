use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

const BLOCKS: [&[(usize, usize)]; 5] = [
    &[(0, 0), (1, 0), (2, 0), (3, 0)],
    &[(1, 2), (0, 1), (1, 1), (2, 1), (1, 0)],
    &[(2, 2), (2, 1), (0, 0), (1, 0), (2, 0)],
    &[(0, 3), (0, 2), (0, 1), (0, 0)],
    &[(0, 1), (1, 1), (0, 0), (1, 0)]
];

fn left(v: &mut Vec<(usize, usize)>, tower: &Vec<Vec<usize>>) {
    if v.iter().all(|(x, y)| *x > 0 && !tower[x - 1].contains(y)) {
        v.iter_mut().for_each(|(x, _)| *x -= 1);
    }
}

fn right(v: &mut Vec<(usize, usize)>, tower: &Vec<Vec<usize>>) {
    if v.iter().all(|(x, y)| *x < 6 && !tower[x + 1].contains(y)) {
        v.iter_mut().for_each(|(x, _)| *x += 1);
    }
}

fn new_block(i: usize, highest: usize) -> Vec<(usize, usize)> {
    BLOCKS[i % 5].iter().map(|(x, y)| (x + 2, y + highest + 4))
        .collect::<Vec<_>>()
}

pub fn run() -> io::Result<()> {
    let mut f = BufReader::new(File::open("data/17.txt")?);
    let mut directions = String::new();
    f.read_line(&mut directions)?;
    let moves = directions.trim().chars().map(|c| match c {
        '<' => &left  as & dyn Fn(&mut Vec<(usize, usize)>, &Vec<Vec<usize>>),
        '>' => &right as & dyn Fn(&mut Vec<(usize, usize)>, &Vec<Vec<usize>>),
        _   => panic!("unreachable")
    });

    let mut tower = vec![vec![0]; 7];
    let mut i = 0;
    let mut block = new_block(i, 0);
    // println!("{}: {:?}", i, block);

    'outer: loop {
        if i % 5 == 0 { 
            let maxs = tower.iter().map(|v| v.iter().max().unwrap()).collect::<Vec<_>>();
            println!("{:?}", maxs);
        }
        println!("inner loop start, {}", i);
        for mv in moves.clone() {
            mv(&mut block, &tower);
            // println!("{}: {:?}", i, block);
            if block.iter().any(|(x, y)| tower[*x].contains(&(y - 1))) {
                block.iter().for_each(|(x, y)| {
                    tower[*x].push(*y);
                    let min = y - 3.min(*y);
                    if tower.iter().all(|v| (min..=*y).any(|y| v.contains(&y))) {
                        tower.iter_mut().for_each(|v| {
                            *v = v.iter().filter(|&&n| n >= min).map(|&n| n).collect();
                        })
                    }
                });
                let max = tower.iter().map(|v| {
                    v.iter().max().unwrap()
                }).max().unwrap();
                i += 1;
                if i == 2022 { println!("17-1: {}", max) }
                if i % 50000000000 == 0 { println!("{}", i) }
                if i == 1000000000000 { println!("17-2: {}", max); break 'outer }
                block = new_block(i, *max);
            } else {
                block.iter_mut().for_each(|(_, y)| *y -= 1);
            }
        }
    }

    Ok(())
}

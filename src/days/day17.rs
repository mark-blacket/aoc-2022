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
const LIMIT: usize = 1000000000000;

struct Cycle { block: usize, i: usize, shape: Vec<usize> }

impl Cycle {
    fn new(block: usize, i: usize, tower: &Vec<Vec<usize>>) -> Self {
        let shape = tower.iter().map(|v| *v.iter().max().unwrap()).collect::<Vec<_>>();
        Cycle { block, i, shape }
    }

    fn diff(&self, other: &Self) -> (usize, Vec<usize>) {
        let shape_diff = self.shape.iter().enumerate().map(|(i, x)| {
            x.abs_diff(other.shape[i])
        }).collect::<Vec<_>>();
        (self.block.abs_diff(other.block), shape_diff)
    }
}

impl PartialEq for Cycle {
    fn eq(&self, other: &Self) -> bool {
        if self.i == other.i && self.block % 5 == other.block % 5 {
            let shapes = [self, other].iter().map(|c| {
                let min = c.shape.iter().min().unwrap();
                c.shape.iter().map(|&x| x - min).collect::<Vec<_>>()
            }).collect::<Vec<_>>();
            shapes[0] == shapes[1]
        } else { false }
    }
}

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

fn add_block(block: &Vec<(usize, usize)>, tower: &mut Vec<Vec<usize>>) -> usize {
    block.iter().for_each(|(x, y)| {
        tower[*x].push(*y);
        let min = y - 3.min(*y);
        if tower.iter().all(|v| (min..=*y).any(|y| v.contains(&y))) {
            tower.iter_mut().for_each(|v| {
                *v = v.iter().filter(|&&n| n >= min).map(|&n| n).collect();
            })
        }
    });
    *tower.iter().map(|v| {
        v.iter().max().unwrap()
    }).max().unwrap()
}

pub fn run() -> io::Result<()> {
    let mut f = BufReader::new(File::open("data/17.txt")?);
    let mut directions = String::new();
    f.read_line(&mut directions)?;
    let moves = directions.trim().chars().map(|c| match c {
        '<' => &left  as & dyn Fn(&mut Vec<(usize, usize)>, &Vec<Vec<usize>>),
        '>' => &right as & dyn Fn(&mut Vec<(usize, usize)>, &Vec<Vec<usize>>),
        _   => panic!("unreachable")
    }).enumerate().cycle();

    let mut tower = vec![vec![0]; 7];
    let mut curr = 0;
    let mut offset = 0;
    let mut block = new_block(curr, 0);
    let mut cycles = Vec::<Cycle>::new();

    for (i, mv) in moves {
        mv(&mut block, &tower);
        if block.iter().any(|(x, y)| tower[*x].contains(&(y - 1))) {
            let mut max = add_block(&block, &mut tower);
            curr += 1;
            if curr > 2022 && offset == 0 {
                let c = Cycle::new(curr, i, &tower);
                if let Some(x) = cycles.iter().rev().find(|&x| x == &c) {
                    let (diff, heights) = c.diff(x);
                    let repeats = (LIMIT - curr) / diff;
                    offset = diff * repeats;
                    tower.iter_mut().enumerate().for_each(|(i, v)| {
                        *v = v.iter().map(|x| {
                            let nx = x + repeats * heights[i];
                            if nx > max { max = nx; }
                            nx
                        }).collect();
                    });
                } else { cycles.push(c) }
            } else if curr == 2022 { println!("17-1: {}", max) }
            if curr + offset == LIMIT { println!("17-2: {}", max); break }
            block = new_block(curr + offset, max);
        } else {
            block.iter_mut().for_each(|(_, y)| *y -= 1);
        }
    }

    Ok(())
}

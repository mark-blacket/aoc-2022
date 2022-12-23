use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::VecDeque;

fn count_bits(mut n: usize) -> usize {
    let mut count = 0;
    while n != 0 {
        n &= n - 1;
        count += 1;
    }
    count
}

fn task1(coords: &Vec<(usize, usize, usize)>, max_c: &(usize, usize, usize)) -> usize {
    let mut area = 0;
    let mut shape = vec![vec![0; max_c.1 + 2];max_c.0 + 2];
    let empty_row = shape[0].clone();
    coords.iter().for_each(|c| shape[c.0][c.1] |= 1 << c.2);
    for (i, row) in shape.iter().enumerate() {
        let upper_row = if i == 0 { &empty_row } else { &shape[i - 1] };
        for (j, cell) in row.iter().enumerate() {
            let upper = upper_row[j];
            let left = if j == 0 { 0 } else { row[j - 1] };
            area += count_bits(upper ^ cell) + count_bits(left ^ cell);
            area += count_bits(cell ^ (cell << 1));
        }
    }
    area
}

fn task2(coords: &Vec<(usize, usize, usize)>, max_c: &(usize, usize, usize)) -> usize {
    let mut area = 0;
    let mut visited = vec![];
    let mut queue = VecDeque::from([(0, 0, 0)]);
    while let Some((x, y ,z)) = queue.pop_front() {
        if visited.contains(&(x, y, z)) { continue }
        visited.push((x, y, z));
        let mut neighbors = vec![];
        if x > 0           { neighbors.push((x - 1, y, z)) }
        if y > 0           { neighbors.push((x, y - 1, z)) }
        if z > 0           { neighbors.push((x, y, z - 1)) }
        if x < max_c.0 + 1 { neighbors.push((x + 1, y, z)) }
        if y < max_c.1 + 1 { neighbors.push((x, y + 1, z)) }
        if z < max_c.2 + 1 { neighbors.push((x, y, z + 1)) }

        for n in neighbors {
            if coords.contains(&n) {
                area += 1;
            } else {
                queue.push_back(n);
            }
        }
    }
    area
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/18.txt")?);
    let mut coords = vec![];
    let mut max_coords = vec![0, 0, 0];

    for line in f.lines() {
        let line = line?.split(',').map(|x| {
            x.parse::<usize>().unwrap() + 1
        }).collect::<Vec<_>>();
        max_coords.iter_mut().enumerate().for_each(|(i, x)| {
            if *x < line[i] { *x = line[i] }
        });
        coords.push((line[0], line[1], line[2]));
    }
    let max_coords = (max_coords[0], max_coords[1], max_coords[2]);

    println!("18-1: {}", task1(&coords, &max_coords));
    println!("18-2: {}", task2(&coords, &max_coords));
    Ok(())
}

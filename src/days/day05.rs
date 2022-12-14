use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

fn task(t: u32) -> io::Result<String> {
    let mut input = vec![
        vec![],
        vec!['s', 't', 'h', 'f', 'w', 'r'],
        vec!['s', 'g', 'd', 'q', 'w'],
        vec!['b', 't', 'w'],
        vec!['d', 'r', 'w', 't', 'n', 'q', 'z', 'j'],
        vec!['f', 'b', 'h', 'g', 'l', 'v', 't', 'z'],
        vec!['l', 'p', 't', 'c', 'v', 'b', 's', 'g'],
        vec!['z', 'b', 'r', 't', 'w', 'g', 'p'],
        vec!['n', 'g', 'm', 't', 'c', 'j', 'r'],
        vec!['l', 'g', 'b', 'w']
    ];
    let f = BufReader::new(File::open("data/5.txt")?);

    for line in f.lines() {
        let v: Vec<usize> = line?.split(' ').skip(1).step_by(2)
            .map(|x| x.parse().unwrap()).collect();
        let l = input[v[1]].len() - v[0];
        let mut moved = input[v[1]].split_off(l);
        if t == 1 {
            moved.reverse();
        }
        input[v[2]].append(&mut moved);
    }

    let s: String = input[1..].iter_mut().map(|x| x.pop().unwrap()).collect();
    Ok(s)
}

pub fn run() -> io::Result<()> {
    println!("5-1: {}", task(1)?.to_uppercase());
    println!("5-2: {}", task(2)?.to_uppercase());
    Ok(())
}

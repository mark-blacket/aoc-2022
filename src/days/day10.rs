use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/10.txt")?);
    let (mut c, mut x) = (0, 1);
    let mut sum = 0;
    let mut crt = String::with_capacity(256);
    
    fn cycle(c: &mut i64, x: &mut i64, sum: &mut i64, crt: &mut String) {
        if *c % 40 == 0 { crt.push('\n'); }
        let cmod = *c % 40;
        *c += 1;
        crt.push(if cmod >= *x - 1 && cmod <= *x + 1 { '#' } else { '.' });
        if [20, 60, 100, 140, 180, 220].contains(&c) { *sum += *c * *x; }
    }

    for line in f.lines() {
        match line?.split_once(' ') {
            None         => cycle(&mut c, &mut x, &mut sum, &mut crt),
            Some((_, n)) => {
                let n = n.parse::<i64>().unwrap();
                cycle(&mut c, &mut x, &mut sum, &mut crt);
                cycle(&mut c, &mut x, &mut sum, &mut crt);
                x += n;
            }
        }
    }

    println!("10-1: {}", sum);
    println!("10-2: {}", crt);
    Ok(())
}

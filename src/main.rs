use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

fn day1() -> io::Result<()> {
    let f = BufReader::new(File::open("data/1.txt")?);
    let mut totals = Vec::new();
    let mut running_sum = 0;

    for line in f.lines() {
        let x = line?;
        if x.is_empty() {
            totals.push(running_sum);
            running_sum = 0;
        } else {
            running_sum += x.parse::<u64>().unwrap();
        }
    }

    totals.sort_unstable_by(|a ,b| b.cmp(a));
    println!("1-1: {}", totals[0]);
    println!("1-2: {}", totals.iter().take(3).sum::<u64>());
    Ok(())
}

fn day2() -> io::Result<()> {
    let f = BufReader::new(File::open("data/2.txt")?);
    let pos1 = ["", "B X", "C Y", "A Z", "A X", "B Y", "C Z", "C X", "A Y", "B Z"];
    let pos2 = ["", "B X", "C X", "A X", "A Y", "B Y", "C Y", "C Z", "A Z" ,"B Z"];
    let mut total1 = 0;
    let mut total2 = 0;

    for line in f.lines() {
        let x = line?;
        total1 += pos1.iter().position(|&p| x.eq(p)).unwrap();
        total2 += pos2.iter().position(|&p| x.eq(p)).unwrap();
    }
    
    println!("2-1: {}", total1);
    println!("2-2: {}", total2);
    Ok(())
}

fn day3_1() -> io::Result<()> {
    let f = BufReader::new(File::open("data/3.txt")?);
    let mut total = 0;
    
    for line in f.lines() {
        let x = line?;
        let (l, r) = x.split_at(x.len() / 2);
        if let Some(c) = l.chars().find(|&c| r.contains(c)) {
            total += if c.is_uppercase() { 26 } else { 0 };
            total += c.to_digit(36).unwrap() - 9;
        }
    }

    println!("3-1: {}", total);
    Ok(())
}

fn day3_2() -> io::Result<()> {
    let f = BufReader::new(File::open("data/3.txt")?);
    let mut total = 0;
    
    let mut buf = [String::new(), String::new(), String::new()];
    for (i, line) in f.lines().enumerate() {
        let x = line?;
        let i = i % 3;
        buf[i] = x;
        if i == 2 {
            let c = buf[0].chars().find(|&c| buf[1].contains(c) && buf[2].contains(c)).unwrap();
            total += if c.is_uppercase() { 26 } else { 0 };
            total += c.to_digit(36).unwrap() - 9;
        };
    }

    println!("3-2: {}", total);
    Ok(())
}

fn day4() -> io::Result<()> {
    let f = BufReader::new(File::open("data/4.txt")?);
    let mut total1 = 0;
    let mut total2 = 0;

    for line in f.lines() {
        let v: Vec<u64> = line?.split(&['-', ','][..]).map(|x| x.parse().unwrap()).collect();
        if (v[0] >= v[2] && v[1] <= v[3]) || (v[0] <= v[2] && v[1] >= v[3]) {
            total1 += 1;
        }
        if !(v[0] > v[3] || v[2] > v[1]) {
            total2 += 1;
        }
    }

    println!("4-1: {}", total1);
    println!("4-2: {}", total2);
    Ok(())
}

fn day5(task: u32) -> io::Result<()> {
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
        if task == 1 {
            moved.reverse();
        }
        input[v[2]].append(&mut moved);
    }

    // println!("{:?}", input);
    let s: String = input[1..].iter_mut().map(|x| x.pop().unwrap()).collect();
    println!("5-{}: {}", task, s.to_uppercase());
    Ok(())
}

fn day6(size: usize, task: u32) -> io::Result<()> {
    let mut s = String::new();
    let mut f = BufReader::new(File::open("data/6.txt")?);
    f.read_line(&mut s)?;

    for start in 0..s.len()-size { 
        let mut found = false;
        s[start..start+size-1].chars().enumerate().for_each(|(i, c)| {
            found |= s[start+i+1..start+size].contains(c);
        });
        if !found {
            println!("6-{}: {}, {}", task, start + size, &s[start..start+size]);
            break;
        }
    }

    Ok(())
}

fn day7() -> io::Result<()> {
    let f = BufReader::new(File::open("data/7.txt")?);
    let mut sizes = HashMap::new();
    let mut path = Vec::new();

    for line in f.lines() {
        let line = line?;
        let v: Vec<&str> = line.split(' ').collect();
        match v[0] {
            "$"   => if v[1] == "cd" {
                match v[2] {
                    ".." => { path.pop(); },
                    x    => { path.push(x.to_owned()); },
                }
            },
            "dir" => (),
            x     => {
                let n = x.parse::<u64>().unwrap();
                for i in 0..path.len() {
                    let dir = path[..=i].join(" ");
                    let size = match sizes.get(&dir) {
                        Some(x) => x + n,
                        None    => n
                    };
                    sizes.insert(dir, size);
                }
            }
        }
    }

    let s: u64 = sizes.values().filter(|&&x| x <= 100000).sum();
    let needed = 30000000 - (70000000 - sizes["/"]);
    let min = sizes.values().filter(|&&x| x >= needed).min().unwrap();
    println!("7-1: {}", s);
    println!("7-2: {}", min);
    Ok(())
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    print!("{}", "Enter day number: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    match input.trim_end().parse().unwrap() {
        1 => day1(),
        2 => day2(),
        3 => { day3_1()?; day3_2() },
        4 => day4(),
        5 => { day5(1)?; day5(2) },
        6 => { day6(4, 1)?; day6(14, 2) },
        7 => day7(),
        _ => {
            println!("Incorrect day number: {}", input.as_str());
            Ok(())
        }
    }
}


use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::{HashMap, HashSet};
// use std::thread;
// use std::time::Duration;

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

fn day8() -> io::Result<()> {
    let f = BufReader::new(File::open("data/8.txt")?);
    let map = f.lines().map(|l| {
        l.unwrap().chars().map(|c| c.to_digit(10).unwrap()).collect::<Vec<_>>()
    }).collect::<Vec<_>>();
    let mut visible = map.len() * 2 + map[0].len() * 2 - 4;
    let mut high_score = 0;

    for (i, row) in map[1..map.len() - 1].iter().enumerate() {
        for (j, val) in row[1..row.len() - 1].iter().enumerate() {
            let col = map.iter().map(|r| r[j + 1]).collect::<Vec<_>>();
            let slices = [&row[..=j], &row[j + 2..], &col[..=i], &col[i + 2..]];

            if slices.iter().any(|it| val > it.iter().max().unwrap()) {
                visible += 1;
            }

            let score = slices.iter().enumerate().map(|(i, x)| {
                let s = if i % 2 == 0 { 
                    x.iter().rev().take_while(|&x| x < val).count()
                } else {
                    x.iter().take_while(|&x| x < val).count()
                };
                if s != x.len() { s + 1 } else { s }
            }).product::<usize>();
            
            if score > high_score {
                high_score = score;
            }
        }
    }

    println!("8-1: {}", visible);
    println!("8-2: {}", high_score);
    Ok(())
}

fn day9(points: usize, task: usize) -> io::Result<()> {
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
    struct Point { x: i64, y: i64 }

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
    
    println!("9-{}: {}", task, pos.len());
    Ok(())
}

fn day10() -> io::Result<()> {
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

fn day11() -> io::Result<()> {
    const LIMIT: u64 = 2 * 3 * 5 * 7 * 11 * 13 * 17 * 19;

    #[derive(Clone)]
    struct Monkey { items: Vec<u64>, div: u64, t: usize, f: usize, sum: usize, op: &'static dyn Fn(u64) -> u64 }

    impl Monkey {
        fn test(&mut self, n: u64) -> (Vec<u64>, Vec<u64>) {
            let (mut t, mut f) = (vec![], vec![]);
            for item in self.items.iter() {
                let item = ((self.op)(*item) / n) % LIMIT;
                if item % self.div == 0 {
                    t.push(item);
                } else {
                    f.push(item);
                }
            }
            self.sum += self.items.len();
            self.items.clear();
            (t, f)
        }
    }

    let mut monkeys = [
        Monkey { items: vec![57, 58],                         div: 7,  t: 2, f: 3, sum: 0, op: &|x| x * 19 },
        Monkey { items: vec![66, 52, 59, 79, 94, 73],         div: 19, t: 4, f: 6, sum: 0, op: &|x| x + 1  },
        Monkey { items: vec![80],                             div: 5,  t: 7, f: 5, sum: 0, op: &|x| x + 6  },
        Monkey { items: vec![82, 81, 68, 66, 71, 83, 75, 97], div: 11, t: 5, f: 2, sum: 0, op: &|x| x + 5  },
        Monkey { items: vec![55, 52, 67, 70, 69, 94, 90],     div: 17, t: 0, f: 3, sum: 0, op: &|x| x * x  },
        Monkey { items: vec![69, 85, 89, 91],                 div: 13, t: 1, f: 7, sum: 0, op: &|x| x + 7  },
        Monkey { items: vec![75, 53, 73, 52, 75],             div: 2,  t: 0, f: 4, sum: 0, op: &|x| x * 7  },
        Monkey { items: vec![94, 60, 79],                     div: 3,  t: 1, f: 6, sum: 0, op: &|x| x + 2  }
    ];

    fn monkey_loop(m: &mut [Monkey], n: u64, reps: usize) -> usize {
        for _ in 0..reps {
            for i in 0..m.len() {
                let (mut t, mut f) = m[i].test(n);
                m[m[i].t].items.append(&mut t);
                m[m[i].f].items.append(&mut f);
            }
        }

        m.sort_unstable_by(|a, b| b.sum.cmp(&a.sum));
        m.iter().map(|m| m.sum).take(2).product::<usize>()
    }

    println!("11-1: {}", monkey_loop(&mut monkeys.clone(), 3, 20));
    println!("11-2: {}", monkey_loop(&mut monkeys, 1, 10000));
    Ok(())
}

fn main() -> io::Result<()> {
    let mut input = String::new();
    print!("{}", "Enter day number: ");
    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    match input.trim_end().parse().unwrap() {
        1  => day1(),
        2  => day2(),
        3  => { day3_1()?; day3_2() },
        4  => day4(),
        5  => { day5(1)?; day5(2) },
        6  => { day6(4, 1)?; day6(14, 2) },
        7  => day7(),
        8  => day8(),
        9  => { day9(2, 1)?; day9(10, 2) },
        10 => day10(),
        11 => day11(),
        _  => {
            println!("Incorrect day number: {}", input.as_str());
            Ok(())
        }
    }
}

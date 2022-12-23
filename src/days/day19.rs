use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
// use std::thread;
// use std::time::Duration;

macro_rules! to_u64 {
    ($x:expr) => { $x.parse::<u64>().unwrap() }
}

struct Blueprint { ore: u64, clay: u64, obsidian: (u64, u64), geode: (u64, u64) }
struct Pack { robots: (u64, u64, u64, u64), res: (u64, u64, u64, u64), mins: usize }

impl Pack {
    fn update(&self, b: &Blueprint, robot: Option<usize>) -> Self {
        let mut res = (
            self.res.0 + self.robots.0,
            self.res.1 + self.robots.1,
            self.res.2 + self.robots.2,
            self.res.3 + self.robots.3
        );
        let mut robots = self.robots.clone();
        match robot {
            Some(0) => { robots.0 += 1; res.0 -= b.ore; },
            Some(1) => { robots.1 += 1; res.0 -= b.clay; },
            Some(2) => { robots.2 += 1; res.0 -= b.obsidian.0; res.1 -= b.obsidian.1 },
            Some(3) => { robots.3 += 1; res.0 -= b.geode.0; res.2 -= b.geode.1 },
            Some(_) => (), None => ()
        }
        Pack { robots, res, mins: self.mins - 1 }
    }
}

fn dfs(b: &Blueprint, mins: usize) -> u64 {
    let mut max_geodes = vec![0; mins];
    let mut stack = Vec::from([
        Pack { robots: (1, 0, 0, 0), res: (0, 0, 0, 0), mins }
    ]);
    let max_ore = *[b.ore, b.clay, b.obsidian.0, b.geode.0].iter().max().unwrap();
    while let Some(p) = stack.pop() {
        if p.mins == 0 { 
            if p.res.3 > max_geodes[0] { max_geodes[0] = p.res.3 }
            continue
        }
        let mut steps = vec![];
        if p.res.0 >= b.geode.0 && p.res.2 >= b.geode.1 {
            steps.push(p.update(&b, Some(3)));
        }
        if p.res.0 >= b.obsidian.0 && p.res.1 >= b.obsidian.1 && p.robots.2 < b.geode.1 {
            steps.push(p.update(&b, Some(2)));
        }
        if p.res.0 >= b.clay && p.robots.1 < b.obsidian.1 {
            steps.push(p.update(&b, Some(1)));
        }
        if p.res.0 >= b.ore && p.robots.0 < max_ore { 
            steps.push(p.update(&b, Some(0)));
        } 
        steps.push(p.update(&b, None));

        for step in steps {
            if step.res.3 >= max_geodes[step.mins] {
                max_geodes[step.mins] = step.res.3;
                stack.push(step);
            }
        }
    }
    max_geodes[0]
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/19.txt")?);
    let mut blueprints = vec![];
    for line in f.lines() {
        let line = line?;
        let v = line.split(' ').collect::<Vec<_>>();
        blueprints.push(Blueprint {
            ore: to_u64!(v[6]),
            clay: to_u64!(v[12]),
            obsidian: (to_u64!(v[18]), to_u64!(v[21])),
            geode: (to_u64!(v[27]), to_u64!(v[30]))
        });
    }

    let result = blueprints.iter().map(|b| dfs(b, 24)).enumerate()
        .fold(0, |acc, (i, x)| acc + (i + 1) as u64 * x);
    println!("19-1: {}", result);
    let result = blueprints.iter().take(3).map(|b| dfs(b, 32)).product::<u64>();
    println!("19-2: {}", result);     
    Ok(())
}

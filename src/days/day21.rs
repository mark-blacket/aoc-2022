use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;
use std::rc::Rc;

enum Op { Add, Sub, Mul, Div }
enum Monkey {
    Num(i64),
    Op(String, String, Op)
}

struct Tree {
    n:      Option<i64>,
    left:   Option<Rc<Tree>>,
    right:  Option<Rc<Tree>>,
    parent: Option<Rc<Tree>>
}
type Monkeys = HashMap<String, Monkey>;

fn solve(map: &Monkeys, name: &String) -> i64 {
    let monkey = map.get(name).unwrap();
    match monkey {
        Monkey::Num(n) => *n,
        Monkey::Op(l, r, op) => match op {
            Op::Add => solve(map, &l, res) + solve(map, &r, res),
            Op::Sub => solve(map, &l, res) - solve(map, &r, res),
            Op::Mul => solve(map, &l, res) * solve(map, &r, res),
            Op::Div => solve(map, &l, res) / solve(map, &r, res),
        }
    }
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/21.txt")?);
    let mut map = HashMap::new();
    for line in f.lines() {
        let line = line?;
        let v = line.split(' ').collect::<Vec<_>>();
        if v.len() == 2 {
            let num = v[1].parse::<i64>().unwrap();
            map.insert(v[0].trim_matches(':').into(), Monkey::Num(num));
        } else {
            let m = Monkey::Op(v[1].into(), v[3].into(), match v[2] {
                "+" => Op::Add,
                "-" => Op::Sub,
                "*" => Op::Mul,
                "/" => Op::Div,
                _ => panic!("unreachable")
            });
            map.insert(v[0].trim_matches(':').into(), m);
        }
    }

    let mut res = HashMap::new();
    println!("21-1: {}", solve(&map, &"root".to_owned(), &mut res));

    Ok(())
}

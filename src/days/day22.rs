use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

const R: usize = 0;
const D: usize = 1;
const L: usize = 2;
const U: usize = 3;
const SIZE: usize = 50;

// yeah i'm a lazy bastard
// .0 - section, .1 - facing
type Conns = [(usize, usize); 4];
const CONNS_1: [Conns; 6] = [
    [(1, R), (2, D), (1, L), (4, U)],
    [(0, R), (1, D), (0, L), (1, U)],
    [(2, R), (4, D), (2, L), (0, U)],
    [(4, R), (5, D), (4, L), (5, U)],
    [(3, R), (0, D), (3, L), (2, U)],
    [(5, R), (3, D), (5, L), (3, U)],
];
const CONNS_2: [Conns; 6] = [
    [(1, R), (2, D), (3, R), (5, R)],
    [(4, L), (2, L), (0, L), (5, U)],
    [(1, U), (4, D), (3, D), (0, U)],
    [(4, R), (5, D), (0, R), (2, R)],
    [(1, L), (5, L), (3, L), (2, U)],
    [(4, U), (1, D), (0, D), (3, U)],
];

struct Section {
    pos: (usize, usize),
    conns: Conns,
    map: Vec<Vec<bool>>,
}

impl Section {
    fn new(v: &Vec<String>, row: usize, col: usize, size: usize) -> Self {
        Section {
            pos: (row, col), conns: [(0, 0); 4],
            map: v[row..row + size].iter().map(|xs| {
                xs[col..col + size].chars().map(|x| match x {
                    '.' => true,
                    '#' => false,
                    _   => panic!("unaccessible")
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>()
        }
    }
}

struct Cursor {
    pos: (usize, usize),
    section: usize,
    facing: usize
}

impl Cursor {
    fn step(&mut self, sections: &Vec<Section>, size: usize) -> bool {
        let (r, c, cmp): (usize, usize, Box<dyn Fn((usize, usize)) -> bool>) = match self.facing {
            R => (0, 1, Box::new(|(_, x)| x == size - 1)),
            D => (1, 0, Box::new(|(x, _)| x == size - 1)),
            L => (0, size - 1, Box::new(|(_, x)| x == 0)),
            U => (size - 1, 0, Box::new(|(x, _)| x == 0)),
            _ => panic!("unaccessible")
        };

        let (section, facing) = if cmp(self.pos) {
            sections[self.section].conns[self.facing]
        } else { (self.section, self.facing) };
        let pos = if facing == self.facing {
            ((self.pos.0 + r) % size, (self.pos.1 + c) % size)
        } else { rotate_pos(self.pos, self.facing, facing, size) };

        if sections[section].map[pos.0][pos.1] {
            *self = Cursor { pos, section, facing };
            false
        } else { true }
    }

    fn turn(&mut self, dir: char) {
        self.facing = match dir {
            'L' => (self.facing + 3) % 4,
            'R' => (self.facing + 1) % 4,
            _   => panic!("wrong direction char")
        };
    }
}

// y = size - x - 1; m = size - 1; 
//            R     D     L     U
// R: x, m -> x, 0; 0, y; y, m; m, x
// D: m, x -> y, 0; 0, x; x, m; m, y
// L: x, 0 -> y, 0; 0, x; x, m; m, y
// U: 0, x -> x, 0; 0, y; y, m; m, x

fn rotate_pos(pos: (usize, usize), of: usize, nf: usize, size: usize) -> (usize, usize) {
    let mut x = if of & 1 == 0 { pos.0 } else { pos.1 };
    let edge = if nf & 2 == 0 { 0 } else { size - 1 };
    if (of ^ nf) & 1 != (of ^ nf) >> 1 { x = size - x - 1; }
    if nf & 1 == 0 { (x, edge) } else { (edge, x) }
}

fn parse_sections(v: &Vec<String>, size: usize, conns: &[[(usize, usize); 4]]) -> Vec<Section> {
    let mut sections = vec![];
    for i in (0..v.len()).step_by(size) {
        (0..v[i].len()).step_by(size)
            .filter(|&j| v[i].chars().nth(j).unwrap() != ' ')
            .for_each(|j| sections.push(Section::new(&v, i, j, size)));
    }
    sections.iter_mut().zip(conns.iter()).for_each(|(s, c)| s.conns = c.clone());
    sections
}

fn parse_path(s: String) -> Vec<(usize, Option<char>)> {
    let mut buf = String::new();
    let mut v = vec![];
    for c in s.chars() {
        if c.is_digit(10) {
            buf.push(c);
        } else {
            v.push((buf.parse::<usize>().unwrap(), Some(c)));
            buf.clear();
        }
    } 
    v.push((buf.parse::<usize>().unwrap(), None));
    v
}

fn solve(s: &Vec<String>, path: &Vec<(usize, Option<char>)>, conns: &[Conns]) -> usize {
    let sections = parse_sections(s, SIZE, conns);
    let mut cursor = Cursor { section: 0, pos: (0, 0), facing: R };

    for (steps, turn) in path {
        for _ in 0..*steps {
            if cursor.step(&sections, SIZE) { break };
        }
        if let Some(c) = *turn {
            cursor.turn(c);
        }
    }

    let r = sections[cursor.section].pos.0 + cursor.pos.0 + 1;
    let c = sections[cursor.section].pos.1 + cursor.pos.1 + 1;
    1000 * r + 4 * c + cursor.facing
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/22.txt")?);
    let mut lines = f.lines().map(|l| l.unwrap());
    let s = lines.by_ref().take_while(|x| !x.is_empty())
        .map(|x| x.to_owned()).collect::<Vec<_>>();
    let path = parse_path(lines.last().unwrap());

    println!("22-1: {}", solve(&s, &path, &CONNS_1));
    println!("22-2: {}", solve(&s, &path, &CONNS_2));
    Ok(())
}

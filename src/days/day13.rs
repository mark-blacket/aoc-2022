use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};

enum Node { Val(i64), Tree(Tree) }
struct Tree { node: Vec<Node> }

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Token { LB(usize), RB(usize), Val(i64) }

fn compare(l: &Tree, r: &Tree) -> Ordering {
    for (nl, nr) in l.node.iter().zip(r.node.iter()) {
        let ord = match (nl, nr) {
            (Node::Val(x),  Node::Val(y))  => x.cmp(&y),
            (Node::Tree(x), Node::Val(y))  => compare(x, &Tree { node: vec![Node::Val(*y)] }),
            (Node::Val(x),  Node::Tree(y)) => compare(&Tree { node: vec![Node::Val(*x)] }, y),
            (Node::Tree(x), Node::Tree(y)) => compare(x, y)
        };
        if ord != Ordering::Equal { return ord }
    }
    l.node.len().cmp(&r.node.len())
}

fn tokenize(s: &str) -> Vec<Token> {
    let mut v = Vec::new();
    let mut buf = String::new();
    let mut count = 0;
    for c in s.chars() {
        match c {
            '[' => {
                v.push(Token::LB(count));
                count += 1;
            },
            ']' => {
                if !buf.is_empty() {
                    v.push(Token::Val(buf.parse::<i64>().unwrap()));
                    buf.clear();
                }
                count -= 1;
                v.push(Token::RB(count));
            },
            ',' => { 
                if !buf.is_empty() {
                    v.push(Token::Val(buf.parse::<i64>().unwrap()));
                    buf.clear();
                }
            },
            _ => buf.push(c)
        }
    }
    v
}

fn parse_line(v: &[Token]) -> Tree {
    // println!("{:?}", v);
    let mut t = Tree { node: vec![] };
    let mut i = 1;
    while i < v.len() {
        match v[i] {
            Token::LB(n)  => {
                let (e, _) = v[i..].iter().enumerate()
                    .find(|(_, &x)| x == Token::RB(n)).unwrap();
                t.node.push(Node::Tree(parse_line(&v[i..e+i+1])));
                i += e;
            },
            Token::RB(_)  => i += 1,
            Token::Val(x) => {
                t.node.push(Node::Val(x));
                i += 1;
            }
        }
    }
    t
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/13.txt")?);
    let mut v = Vec::new();
    let (mut i, mut sum) = (0, 0);
    for line in f.lines() {
        let line = line?;
        if line.is_empty() {
            if compare(&v[i * 2], &v[i * 2 + 1]) == Ordering::Less { 
                sum += i + 1;
            }
            i += 1;
        } else {
            v.push(parse_line(&tokenize(&line)));
        }
    }

    v.sort_by(compare);
    let divs = ["[[2]]", "[[6]]"].iter().map(|x| {
        let div = parse_line(&tokenize(x));
        for (i, t) in v.iter().enumerate() {
            if compare(t, &div) == Ordering::Greater {
                return i + 1;
            }
        }
        0
    }).collect::<Vec<_>>();

    println!("13-1: {}", sum);
    println!("13-2: {}", divs[0] * (divs[1] + 1));
    Ok(())
}

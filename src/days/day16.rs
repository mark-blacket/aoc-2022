use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct Valve { rate: i64, paths: Vec<String> }
#[derive(Debug)]
struct Path  { score: i64, path: Vec<String> }

impl Path {
    fn new(visited: &Vec<(&str, i64)>, valves: &HashMap<String, Valve>) -> Self {
        Path {
            score: visited.iter().map(|(k, v)| valves[*k].rate * v).sum::<i64>(),
            path:  visited.iter().map(|(k, _)| (*k).to_owned()).collect::<Vec<_>>()
        }
    }
}

type DistMap = HashMap<String, HashMap<String, i64>>;

fn floyd_warshall(map: &HashMap<String, Valve>) -> DistMap {
    let mut dist: DistMap = HashMap::from_iter(
        map.keys().map(|x| (x.clone(), HashMap::from_iter(
            map[x].paths.iter().map(|k| (k.clone(), 1))
                .chain([(x.clone(), 0)].into_iter())
        )))
    );

    for k in map.keys() { for i in map.keys() { for j in map.keys() {
        if let (Some(&ik), Some(&kj)) = (dist[i].get(k), dist[k].get(j)) {
            if dist[i].get(j).unwrap_or(&i64::MAX) > &(ik + kj) {
                dist.get_mut(i).unwrap().insert(j.clone(), ik + kj);
            }
        }
    }}}

    dist
}

fn solutions(start: &str, steps: i64, valves: &HashMap<String, Valve>,
             dist: &DistMap, visited: Vec<(&str, i64)>) -> Vec<Path> {
    let mut vec = vec![];
    let filtered = valves.iter().filter(|(k, v)| {
        v.rate > 0 && !visited.iter().any(|(x, _)| k == x)
    }).collect::<Vec<_>>();
    vec.push(Path::new(&visited, valves));
    for (k, _) in filtered {
        let s = steps - dist[start][k] - 1;
        if s > 0 {
            let mut visited = visited.clone();
            visited.push((k, s));
            vec.append(&mut solutions(k, s, valves, dist, visited));
        }
    }
    vec
}

pub fn run() -> io::Result<()> {
    let f = BufReader::new(File::open("data/16.txt")?);
    let mut valves = HashMap::new();
    for line in f.lines() {
        let line = line?;
        let v = line.split(' ').collect::<Vec<_>>();
        valves.insert(v[1].to_owned(), Valve {
            rate:  v[4].split_once('=').unwrap().1.trim_end_matches(';')
                .parse::<i64>().unwrap(),
            paths: v[9..].iter().map(|x| x.trim_end_matches(',').to_owned())
                .collect::<Vec<_>>()
        });
    }
    let dist = floyd_warshall(&valves);

    let sols1 = solutions("AA", 30, &valves, &dist, vec![]);
    let max = sols1.iter().max_by(|a, b| a.score.cmp(&b.score)).unwrap();
    println!("16-1: {}, {}", max.score, max.path.join("->"));

    let mut pairs = vec![];
    let mut sols2 = solutions("AA", 26, &valves, &dist, vec![]);
    let mut max_pair = (0, &vec![], &vec![]);
    sols2.sort_by(|a, b| b.score.cmp(&a.score));
    for i in 0..sols2.len() - 1 {
        for j in i + 1..sols2.len() {
            if !sols2[i].path.iter().any(|x| sols2[j].path.contains(x)) {
                let score = sols2[i].score + sols2[j].score;
                pairs.push((score, &sols2[i].path, &sols2[j].path));
            }
        }
        if let Some(x) = pairs.iter().max_by(|a, b| a.0.cmp(&b.0)) {
            if x.0 == max_pair.0 { break } else { max_pair = *x }
        }
    }
    println!("16-2: {}, {}, {}", pairs[0].0, pairs[0].1.join("->"), pairs[0].2.join("->"));

    Ok(())
}

use std::io;

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

pub fn run() -> io::Result<()> {
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

    println!("11-1: {}", monkey_loop(&mut monkeys.clone(), 3, 20));
    println!("11-2: {}", monkey_loop(&mut monkeys, 1, 10000));
    Ok(())
}

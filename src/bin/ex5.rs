use std::fs::read_to_string;
use core::str::Lines;
use std::iter::Peekable;
use std::num::ParseIntError;
use std::str::FromStr;

struct Almanac {
    des_range: u64,
    source_range: u64,
    range: u64,
}

impl FromStr for Almanac {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let res: Vec<u64> = line.split(' ').map(|x|x.parse::<u64>()).flatten().collect();
        return Ok(Almanac {des_range: res[0], source_range: res[1], range: res[2]});
    }
}

fn read_seeds(seed_line: &str) -> Vec<u64>{
    println!("{}", seed_line);
    let (_, seeds_str) = seed_line.split_once(':').unwrap();
    return seeds_str.split(' ').map(|x|x.parse::<u64>()).flatten().collect();
}

fn read_next_almanac(mut lines: &mut Peekable<Lines>) -> Vec<Almanac> {
    println!("title: {}", lines.next().unwrap());
    let mut almanac: Vec<Almanac> = Vec::new();
    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        if line.is_empty(){
            println!("I am empty");
            break;
        }
        println!("found a line");
        almanac.push(Almanac::from_str(line).unwrap());
    }
    return almanac
}

fn matching(next_id: u64, step: &Vec<Almanac>) -> Option<u64> {
    for almanac in step {
        println!("matching{}<={}<{}", almanac.source_range, next_id, almanac.source_range + almanac.range);
        if almanac.source_range <= next_id && next_id < almanac.source_range + almanac.range {
            let offset = next_id - almanac.source_range;
            println!("offset {}", offset);
            return Some(almanac.des_range+offset)
        }
    }
    return None
}

fn walk_over_alamnacs(seed_id: u64, steps: &Vec<Vec<Almanac>>) -> u64{
    println!("seeeed: {}", seed_id);
    let mut next_id = seed_id;
    for step in steps {
        match matching(next_id, step) {
            None => {println!("no match, taking its id:{}", next_id);}
            Some(res) => {
                println!("gogogo next: {}", res);
                next_id = res;
            }
        }

    }
    println!("final id:{}", next_id);
    return next_id;
}

fn main() {
    let filename = "5.in";
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines().peekable();
    let seeds = read_seeds(lines.next().unwrap());
    // for seed in seeds {
    //     println!("{}", seed)
    // }
    lines.next();
    let mut steps: Vec<Vec<Almanac>> = Vec::new();
    while lines.peek().is_some() {
        steps.push(read_next_almanac(&mut lines))
    }
    let result: u64 = seeds.iter().map(|x| walk_over_alamnacs(*x, &steps)).min().unwrap();
    println!("restult {}", result)
}
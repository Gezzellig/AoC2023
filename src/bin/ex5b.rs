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

fn read_seeds_list(seed_line: &str) -> Vec<u64>{
    println!("{}", seed_line);
    let (_, seeds_str) = seed_line.split_once(':').unwrap();
    let seeds: Vec<u64> = seeds_str.split(' ').map(|x|x.parse::<u64>()).flatten().collect();
    let mut actual_seeds: Vec<u64> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        for j in seeds[i]..(seeds[i] + seeds[i+1]) {
            actual_seeds.push(j);
        }
        // actual_seeds.append((seeds[i]..seeds[i+1]).cloned().collect());
    }
    return actual_seeds
}

fn read_next_almanac(mut lines: &mut Peekable<Lines>) -> Vec<Almanac> {
    println!("title: {}", lines.next().unwrap());
    let mut almanac: Vec<Almanac> = Vec::new();
    while lines.peek().is_some() {
        let line = lines.next().unwrap();
        if line.is_empty(){
            break;
        }
        almanac.push(Almanac::from_str(line).unwrap());
    }
    return almanac
}

fn matching_reverse(next_id: u64, step: &Vec<Almanac>) -> Option<u64> {
    for almanac in step {
        if almanac.des_range <= next_id && next_id < almanac.des_range + almanac.range {
            let offset = next_id - almanac.des_range;
            return Some(almanac.source_range+offset)
        }
    }
    return None
}

fn walk_over_alamnacs_reverse(location_id: u64, steps: &Vec<Vec<Almanac>>) -> u64{
    let mut next_id = location_id;
    for step in steps {
        match matching_reverse(next_id, step) {
            None => {}
            Some(res) => {
                next_id = res;
            }
        }
    }
    return next_id;
}

fn test_match(seeds: &Vec<u64>, result: u64) -> bool {
    for i in (0..seeds.len()).step_by(2) {
        // println!("{} <= {} < {}", seeds[i], result, seeds[i] + seeds[i+1]);
        if seeds[i] <= result && result < (seeds[i] + seeds[i+1]) {
            return true
        }
    }
    return false
}

fn main() {
    let filename = "5.in";
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines().peekable();
    let seeds = read_seeds(lines.next().unwrap());
    //
    // for seed in seeds {
    //     println!("{}", seed)
    // }
    lines.next();
    let mut steps: Vec<Vec<Almanac>> = Vec::new();
    while lines.peek().is_some() {
        steps.push(read_next_almanac(&mut lines))
    }
    steps.reverse();

    let mut x: u64 = 0;
    while true {
        x +=1;
        if x%2000000 == 0 {
            println!("hiero nu {}", x)
        }
        let result = walk_over_alamnacs_reverse(x, &steps);
        // println!("result {}", result);
        if test_match(&seeds, result) {
            break;
        }
    }
    println!("x: {}", x);
    // let result: u64 = seeds.iter().map(|x| walk_over_alamnacs(*x, &steps)).min().unwrap();
    // println!("restult {}", result)
}
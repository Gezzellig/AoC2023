use std::fs::read_to_string;
use itertools::Itertools;

const SIZE: usize = 140;
const EXPANSION: usize = 1000000-1;

fn line_to_galaxies(line_index: usize, line: &str) -> Vec<(usize, usize)> {
    println!("{}", line);
    line.chars().enumerate().filter(|(_, c)| *c == '#').map(|(x,_)|(x, line_index)).collect()
}

fn expand_y(galaxies: &mut Vec<(usize, usize)>) {
    let mut extra = 0;
    let mut cur_galaxy = 0;
    for i in 0..SIZE {
        let mut m = false;
        while cur_galaxy < galaxies.len() && galaxies[cur_galaxy].1 == i {
            galaxies[cur_galaxy].1 += extra;
            cur_galaxy += 1;
            m = true;
        }
        if !m {
            println!("EXTRA after {}", i);
            extra+=EXPANSION;
        }
    }
}

fn expand_x(galaxies: &mut Vec<(usize, usize)>) {
    galaxies.sort_by(|(x1, _),(x2,_)|x1.cmp(x2));
    println!("sorted{:?}", galaxies);
    let mut extra = 0;
    let mut cur_galaxy = 0;
    for i in 0..SIZE {
        let mut m = false;
        while cur_galaxy < galaxies.len() && galaxies[cur_galaxy].0 == i {
            galaxies[cur_galaxy].0 += extra;
            cur_galaxy += 1;
            m = true;
        }
        if !m {
            println!("EXTRA after {}", i);
            extra+=EXPANSION;
        }
    }
}

fn calc_manhatten_distance(a: (usize, usize), b: (usize, usize)) -> usize {
    ((a.0 as i32 - b.0 as i32).abs() + (a.1 as i32 - b.1 as i32).abs()) as usize
}

fn main() {
    let filename = "11.in";
    let mut galaxies: Vec<_> = read_to_string(filename).unwrap().lines().enumerate().map(|(i, line)|line_to_galaxies(i, line)).flatten().collect();
    println!("{:?}", galaxies);
    expand_y(&mut galaxies);
    println!("{:?}", galaxies);
    expand_x(&mut galaxies);
    println!("{:?}", galaxies);
    let result: usize = galaxies.iter().combinations(2).map(|g|calc_manhatten_distance(*g[0], *g[1])).sum();
    println!("{}", result);
}
use std::fs::read_to_string;
use std::iter::{Peekable, Repeat};
use std::str::Lines;

// const LR_SIZE: usize = 3;
const ZZZ: usize = 252525*2;
const OPTION_SIZE: usize = ZZZ+2;

fn create_lr_array(line: &str) -> Vec<usize> {
    let lr_array: Vec<_> = line.chars().map(|x|if x == 'L' {0} else {1}).collect::<Vec<usize>>().try_into().unwrap();
    println!("{:?}", lr_array);
    return lr_array;
}

fn text_to_number(text: &str) -> usize {
    let c_list: Vec<char> = text.chars().collect();
    let result = (c_list[0] as usize - 'A' as usize) * 10000
        +(c_list[1] as usize - 'A' as usize) * 100
        +(c_list[2] as usize - 'A' as usize);
    return result
}

fn is_start(num: usize) -> bool {
    num % 100 == 0
}

fn is_finish(num: usize) -> bool {
    num % 100 == 25*2
}

fn parse_walk_line(line: &str) -> (usize, usize, usize) {
    let (source_str, other_str) = line.split_once(" = ").unwrap();
    let source = text_to_number(source_str);
    let mut cleaned_str = other_str.replacen('(', "", 1);
    cleaned_str = cleaned_str.replacen(')', "", 1);
    let (left_str, right_str) = cleaned_str.split_once(", ").unwrap();
    let left = text_to_number(left_str);
    let right = text_to_number(right_str);
    if is_finish(source*2) {
        println!("its finish: {}", source_str)
    }
    return (source*2, left*2, right*2);
}

fn create_walk_array(mut lines: &mut Lines) -> [usize; OPTION_SIZE] {
    let mut walk_array = [0; OPTION_SIZE];
    // let mut start_positions = Vec::new();
    for line in lines {
        let (source, left, right) = parse_walk_line(line);
        println!("{} -> {} or {}", source, left, right);
        if is_start(source) {
            println!("ITs a STARTTT");
            // start_positions.push(source);
        }
        if is_finish(source) {
            println!("ITs a FINISHHH");
        }
        walk_array[source] = left;
        walk_array[source+1] = right;
    }
    return walk_array
}

fn find_start_positions(mut lines: &mut Lines) -> Vec<usize> {
    let mut start_positions = Vec::new();
    for line in lines {
        let (source, left, right) = parse_walk_line(line);
        if is_start(source) {
            println!("ITs a STARTTT");
            start_positions.push(source);
        }
        if is_finish(source) {
            println!("ITs a FINISHHH");
        }
    }
    return start_positions
}

fn find_end_positions(mut lines: &mut Lines) -> Vec<usize> {
    let mut start_positions = Vec::new();
    for line in lines {
        let (source, left, right) = parse_walk_line(line);
        if is_start(source) {
            println!("ITs a STARTTT");

        }
        if is_finish(source) {
            println!("ITs a FINISHHH");
            start_positions.push(source);
        }
    }
    return start_positions
}

fn walk_over_array(lr_array: &Vec<usize>, walk_array: &[usize; OPTION_SIZE], mut positions: Vec<usize>) -> usize{
    let mut counter = 0;
    println!("Going here");

    loop {
        for step in lr_array {
            if counter % 20000000 == 0 {
                println!("{}", counter)
            }
            // println!("{:?}, step: {}", positions, step);
            if positions.iter().map(|&x| is_finish(x)).all(|x|x) {
                return counter;
            }
            positions = positions.iter().map(|x|walk_array[x + step]).collect();

            counter += 1;
        }
    }
    return 0;
}
#[derive(Debug)]
struct Finish {
    i_step: usize,
    location: usize,
    steps_till_here: u64,
}

impl PartialEq for Finish {
    fn eq(&self, other: &Finish) -> bool {
        return self.i_step == other.i_step && self.location == other.location
    }
}

fn find_cycles_with_finishes(lr_array: &Vec<usize>, walk_array: &[usize; OPTION_SIZE], start_position: usize) {
    let mut cur_loc = start_position;
    let mut total_steps:u64 = 0;
    let mut finishes: Vec<Finish> = Vec::new();
    loop {
        for i_step in 0..lr_array.len() {
            if is_finish(cur_loc) {
                let cur_finish = Finish{i_step, location: cur_loc, steps_till_here: total_steps };
                if let Some(old_finish) = finishes.iter().find(|f| **f==cur_finish) {
                    println!("FOUND CYCLE");
                    println!("old: {:?}", old_finish);
                    println!("new: {:?}", cur_finish);

                    // To get the answer calculate the Least common multiple of the different starting positions
                    println!("need {} steps to get to cycle, and cycle takes {} steps", old_finish.steps_till_here, cur_finish.steps_till_here-old_finish.steps_till_here);
                    println!("all finishes: {:?}", finishes);
                    return;
                    // TODO: calc cycle specs
                }
                finishes.push(cur_finish);
            }
            let step = lr_array[i_step];
            // println!("Going from {} taking {} to {}", cur_loc, step, walk_array[cur_loc + step]);
            cur_loc = walk_array[cur_loc + step];
            total_steps += 1;
            // println!("{:?}, step: {}", positions, step);
        }
    }
}

fn main() {
    // let filename = "8bsmall.in";
    let filename = "8.in";
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines();
    let lr_array = create_lr_array(lines.next().unwrap());
    lines.next();
    let walk_array = create_walk_array(&mut lines.clone());
    let start_positions = find_start_positions(&mut lines.clone());
    // let end_positions = find_end_positions(&mut lines);
    for start_pos in start_positions {
        println!("calculating cycles for: {}", start_pos);
        find_cycles_with_finishes(&lr_array, &walk_array, start_pos)
    }
    // println!("{:?}", end_positions);
    // let result = walk_over_array(&lr_array, &walk_array, start_positions);
    // println!("result: {}", result)
}
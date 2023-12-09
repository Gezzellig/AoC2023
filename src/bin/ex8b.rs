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

fn main() {
    let filename = "8.in";
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines();
    let lr_array = create_lr_array(lines.next().unwrap());
    lines.next();
    let walk_array = create_walk_array(&mut lines.clone());
    let start_positions = find_start_positions(&mut lines.clone());
    // let end_positions = find_end_positions(&mut lines);
    println!("{:?}", start_positions);
    // println!("{:?}", end_positions);
    let result = walk_over_array(&lr_array, &walk_array, start_positions);
    // println!("result: {}", result)
}
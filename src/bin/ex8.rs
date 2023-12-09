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

fn parse_walk_line(line: &str) -> (usize, usize, usize) {
    let (source_str, other_str) = line.split_once(" = ").unwrap();
    let source = text_to_number(source_str);
    let mut cleaned_str = other_str.replacen('(', "", 1);
    cleaned_str = cleaned_str.replacen(')', "", 1);
    let (left_str, right_str) = cleaned_str.split_once(", ").unwrap();
    let left = text_to_number(left_str);
    let right = text_to_number(right_str);
    return (source, left, right);
}

fn create_walk_array(mut lines: &mut Peekable<Lines>) -> [usize; OPTION_SIZE]{
    let mut walk_array = [0; OPTION_SIZE];
    for line in lines {
        let (source, left, right) = parse_walk_line(line);
        println!("{} -> {} or {}", source, left, right);
        walk_array[source*2] = left*2;
        walk_array[source*2+1] = right*2;
    }
    return walk_array
}

fn walk_over_array(lr_array: &Vec<usize>, walk_array: &[usize; OPTION_SIZE]) -> usize{
    let mut counter = 0;
    let mut cur_loc = 0;
    println!("Going here");

    loop {
        for step in lr_array {
            if cur_loc == ZZZ {
                return counter;
            }
            println!("Going from {} taking {} to {}", cur_loc, step, walk_array[cur_loc + step]);
            cur_loc = walk_array[cur_loc + step];
            counter += 1;
        }
    }
}

fn main() {
    let filename = "8.in";
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines().peekable();
    let lr_array = create_lr_array(lines.next().unwrap());
    lines.next();
    let walk_array = create_walk_array(&mut lines);
    let result = walk_over_array(&lr_array, &walk_array);
    println!("result: {}", result)
}
use std::fs::read_to_string;

// Possible optimisation, use arrays instead of Vec
const LINE_LENGTH: usize = 21;

fn parse_line(line: &str) -> [i32; LINE_LENGTH] {
    let sequence: [i32; LINE_LENGTH] = line.split(' ').map(|x|x.parse().unwrap()).rev().collect::<Vec<i32>>().try_into().unwrap();
    println!("{:?}", sequence);
    return sequence;
}

fn all_zeros(sequence: &[i32; LINE_LENGTH], length: usize) -> bool{
    for i in 0..length {
        if sequence[i] != 0 {
            return false
        }
    }
    return true
}

fn calc_difference_inline(sequence: &mut [i32; LINE_LENGTH], length: usize) {
    for i in 0..length-1 {
        sequence[i] = sequence[i+1] - sequence[i];
    }
}

fn difference_loop(sequence: &mut [i32; LINE_LENGTH]) -> usize {
    let mut length = LINE_LENGTH;
    loop {
        if all_zeros(&sequence, length) {
            println!("ALLL zeros");
            return length;
        }
        calc_difference_inline(sequence, length);
        println!("{}: {:?}", length, sequence);
        length -= 1;
    }
}
fn calculate_line(line: &str) -> i32{
    let mut sequence = parse_line(line);
    let start_sequence = difference_loop(&mut sequence);
    println!("{}: {:?}", start_sequence, sequence);

    let mut new_val = 0;
    for i in start_sequence..LINE_LENGTH {
        new_val += sequence[i];
        println!("new val:{}", new_val);
        // result += new_val;
    }
    println!("this result: {}", new_val);
    return new_val;
}

fn main() {
    let filename = "9.in";
    let result: i32 = read_to_string(filename).unwrap().lines().map(calculate_line).sum();
    println!("{}", result);
}
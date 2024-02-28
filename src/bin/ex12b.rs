use std::fs::read_to_string;
use itertools::Itertools;

fn early_evaluation(wells: &Vec<char>, info: &Vec<usize>) -> bool {
    let mut info_index = 0;
    let mut wells_iter = wells.iter();
    while let Some(c) = wells_iter.next() {
        if *c == '?' {
            return true
        }
        if *c == '#' {
            if info_index == info.len() {
                return false
            }
            for _ in 1..info[info_index] {
                if let Some(c) = wells_iter.next() {
                    if *c == '?' {
                        return true
                    } else if *c != '#' {
                        return false
                    }
                } else {
                    return false
                }
            }
            // Next element should be a `.` or end of well
            if let Some(c) = wells_iter.next() {
                if *c == '?' {
                    return true
                } else if *c != '.' {
                    return false
                }
            }
            info_index += 1;
        }
    }
    return true
}

fn evaluate_wells(wells: &Vec<char>, info: &Vec<usize>) -> bool {
    let mut info_index = 0;
    let mut wells_iter = wells.iter();
    while let Some(c) = wells_iter.next() {
        if *c == '#' {
            if info_index == info.len() {
                return false
            }
            for _ in 1..info[info_index] {
                if let Some(c) = wells_iter.next() {
                    if *c != '#' {
                        return false
                    }
                } else {
                    return false
                }
            }
            // Next element should be a `.` or end of well
            if let Some(c) = wells_iter.next() {
                if *c != '.' {
                    return false
                }
            }
            info_index += 1;
        }
    }
    return info_index == info.len()
}

fn gen_option(wells: &Vec<char>, info: &Vec<usize>, index: usize) -> usize{
    if index == wells.len() {
        // println!("time to evaluate: {:?}", wells);
        return if evaluate_wells(wells, info) {
            // println!("evauated true: {:?} {:?}", wells, info);
            1
        } else {
            0
        }
    }
    if !early_evaluation(wells, info) {
        return 0
    }
    if wells[index] == '?' {
        let mut duplicate = wells.clone();
        let mut result = 0;
        duplicate[index] = '#';
        result += gen_option(&duplicate, info, index+1);
        duplicate[index] = '.';
        result += gen_option(&duplicate, info, index+1);
        return result;
    } else {
        return gen_option(wells, info, index+1);
    }
}

fn calculate_line(line: &str) -> usize {
    println!("{}", line);
    let (wells_str, info_str) = line.split_once(' ').unwrap();
    let info_small: Vec<_> = info_str.split(',').flat_map(|x|x.parse::<usize>()).collect();
    let info = info_small.iter().cycle().take(info_small.len() * 5).map(|x|*x).collect_vec();
    let mut wells_small: Vec<_> = wells_str.chars().collect_vec();
    wells_small.push('?');
    let mut wells = wells_small.iter().cycle().take(wells_small.len() * 5 -1).map(|x|*x).collect_vec();
    wells.pop();
    let wells_str2: String = wells.clone().into_iter().collect();
    println!("{:?}", wells_str2);
    println!("{:?}", info);
    let result = gen_option(&wells, &info, 0);
    println!("result of line {:?} {:?}: {}", wells, info, result);
    return result
    // return 0
}

fn main() {
    let filename = "12.in";
    let result: usize = read_to_string(filename).unwrap().lines().map(calculate_line).sum();
    println!("result: {}", result)
}
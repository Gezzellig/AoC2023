use std::fs::read_to_string;
use std::iter::Peekable;
use std::str::Lines;
use itertools::Itertools;

fn check_row_pos_eq(row: &[char; MAX_COL], dim: (usize, usize), pos: usize) -> bool {
    println!{"trying: {}", pos}
    let mut index: usize = 0;
    while pos as i32-index as i32 >= 0 && pos+index+1 < dim.0 {
        // println!{"comparing: {}=={}", row[pos-index], row[pos+index+1]}
        if row[pos-index] != row[pos+index+1] {
            return false
        }
        index += 1;
    }
    true
}

fn check_column_pos_eq(cube: &[[char; MAX_COL]; MAX_ROW], dim: (usize, usize), column: usize, pos: usize) -> bool {
    println!{"trying: {}", pos}
    let mut index: usize = 0;
    while pos as i32-index as i32 >= 0 && pos+index+1 < dim.1 {
        // println!{"comparing: {}=={}", cube[pos-index][column], cube[pos+index+1][column]}
        if cube[pos-index][column] != cube[pos+index+1][column] {
            return false
        }
        index += 1;
    }
    true
}

fn check_row_eq(row: &[char; MAX_COL], dim: (usize, usize), posible_options: Vec<usize>) -> Vec<usize> {
    return posible_options.iter().filter(|pos| check_row_pos_eq(row, dim, **pos)).map(|x|*x).collect_vec()
}

fn check_column_eq(cube: &[[char; MAX_COL]; MAX_ROW], dim: (usize, usize), column: usize, posible_options: Vec<usize>) -> Vec<usize> {
    return posible_options.iter().filter(|pos| check_column_pos_eq(cube, dim, column, **pos)).map(|x|*x).collect_vec()
}

fn print_cube(cube: &mut [[char; MAX_COL]; MAX_ROW], dim: (usize, usize)) {
    println!("size: {:?}", dim);
    for y in 0..dim.1 {
        let s: String = cube[y].iter().take(dim.0).collect();
        println!("{}", s);
    }
}

fn read_cube(lines: &mut Peekable<Lines>, cube: &mut [[char; MAX_COL]; MAX_ROW]) -> (usize, usize){
    let mut width = 0;
    let mut height = 0;
    while let Some(line) = lines.next() {
        if line == "" {
            break
        }
        line.chars().enumerate().for_each(|(i, c)|cube[height][i] = c);
        width = line.len();
        height += 1;
    }
    return (width, height)
}

fn check_rows(cube: &[[char; MAX_COL]; MAX_ROW], dim: (usize, usize)) -> Option<usize>{
    let mut row = 0;
    let mut res = (0..dim.0-1).collect_vec();
    while res.len() > 0 && row < dim.1 {
        res = check_row_eq(&cube[row], dim, res);
        row += 1;
    }
    println!("ressss: {:?}", res);
    if res.len() == 1 {
        return Some(res[0]+1)
    } else {
        return None
    }
}

fn check_columns(cube: &[[char; MAX_COL]; MAX_ROW], dim: (usize, usize)) -> Option<usize>{
    let mut column = 0;
    let mut res = (0..dim.1-1).collect_vec();
    while res.len() > 0 && column < dim.0 {
        res = check_column_eq(cube, dim, column, res);
        column += 1;
    }
    println!("ressss: {:?}", res);
    if res.len() == 1 {
        return Some((res[0]+1)*100)
    } else {
        return None
    }
}

fn process_cube(lines: &mut Peekable<Lines>, cube: &mut [[char; MAX_COL]; MAX_ROW]) -> usize {
    let dim = read_cube(lines, cube);
    print_cube(cube, dim);
    if let Some(n) = check_rows(cube, dim) {
        println!("FOUND row equality: {}", n);
        return n
    }
    return check_columns(cube, dim).unwrap();
}

const MAX_COL: usize = 18;
const MAX_ROW: usize = 18;


fn main() {
    let filename = "13.in";
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines().peekable();
    let mut cube = [['?';MAX_COL]; MAX_ROW];
    let mut result = 0;
    while lines.peek().is_some() {
        result += process_cube(&mut lines, &mut cube)
    }
    println!("result: {}", result);
}
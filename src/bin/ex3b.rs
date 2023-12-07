use std::fs::read_to_string;

const ROWS: usize = 140;
const COLS: usize = 140;
// const ROWS: usize = 10;
// const COLS: usize = 10;

fn in_bounds(x: i32, y: i32) -> bool{
    return x > -1 && y > -1 && x < COLS as i32 && y < ROWS as i32
}

fn print_matrix(matrix: [[char; crate::COLS]; crate::ROWS]) {
    for row in matrix.iter() {
        for c in row.iter() {
            print!("{}", c);
        }
        print!("\n")
    }
}

fn find_number_start(matrix: [[char; crate::COLS]; crate::ROWS], x: usize, y: usize) -> usize{
    let mut x_n = x as i32;
    while in_bounds(x_n, y as i32) && matrix[y][x_n as usize].is_digit(10) {
        x_n += -1;
    }
    return (x_n+1) as usize
}

fn parse_number(matrix: [[char; crate::COLS]; crate::ROWS], mut x: usize, y: usize) -> u32 {
    let mut number = 0;
    while in_bounds(x as i32, y as i32) && matrix[y][x].is_digit(10) {
        number = number*10 + matrix[y][x].to_digit(10).unwrap();
        x += 1;
    }
    println!("{}", number);
    return number
}

fn find_neighbouring_numbers(matrix: [[char; crate::COLS]; crate::ROWS], x: usize, y: usize) -> Vec<(usize, usize)>{
    let mut number_starts: Vec<(usize, usize)> = Vec::new();
    for i in y-1..=y+1 {
        for j in x-1..=x+1 {
            if matrix[i][j].is_digit(10) {
                let x_n = find_number_start(matrix, j, i);
                let new = (x_n, i);
                if !number_starts.contains(&new) {
                    number_starts.push(new);
                }
            }
        }
    }
    return number_starts
}

fn find_number_starts(matrix: [[char; crate::COLS]; crate::ROWS]) -> u32 {
    let mut result = 0;
    for (y, row) in matrix.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '*' {
                let starts = find_neighbouring_numbers(matrix, x, y);
                if starts.len() == 2 {
                    print!("!");
                    result += parse_number(matrix, starts[0].0, starts[0].1) * parse_number(matrix, starts[1].0, starts[1].1)
                } else {
                    print!(".");
                }
            } else {
                print!(".");
            }
        }
        print!("\n")
    }
    return result
}

fn read_file(filename: &str) {
    // 140 wide 140 long
    // Define the dimensions of the matrix


    // Create a 2D array
    let mut matrix: [[char; COLS]; ROWS] = [['?'; COLS]; ROWS];
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        for (j, char_value) in line.chars().enumerate() {
            matrix[i][j] = char_value;
        }
    }
    print_matrix(matrix);
    let result = find_number_starts(matrix);
    println!("HAHAHAH");
    println!("result: {}", result);
}

fn main() {
    read_file("3.in")
}
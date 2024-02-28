use std::fs::read_to_string;

const SIZE: usize = 140;

#[derive(PartialEq, Debug)]
enum Dir {
    NORTH,
    EAST,
    SOUTH,
    WEST,
}

fn dir_to_val(dir: &Dir) -> (i32, i32){
    match dir {
        Dir::NORTH => (0,-1),
        Dir::EAST => (1,0),
        Dir::SOUTH => (0,1),
        Dir::WEST => (-1,0),
    }
}

fn oposite_dir(dir: &Dir) -> Dir {
    match dir {
        Dir::NORTH => Dir::SOUTH,
        Dir::EAST => Dir::WEST,
        Dir::SOUTH => Dir::NORTH,
        Dir::WEST => Dir::EAST,
    }
}

fn char_to_dir(c: char) -> Option<(Dir, Dir)> {
    match c {
        '|' => Some((Dir::NORTH, Dir::SOUTH)),
        '-' => Some((Dir::EAST, Dir::WEST)),
        'L' => Some((Dir::NORTH, Dir::EAST)),
        'J' => Some((Dir::NORTH, Dir::WEST)),
        '7' => Some((Dir::SOUTH, Dir::WEST)),
        'F' => Some((Dir::SOUTH, Dir::EAST)),
        _ => {
            println!("Somethings going wrong, received: {}", c);
            return None
        }
    }
}

fn get_out_dir(c: char, in_dir: &Dir) -> Option<Dir> {
    if let Some((a, b)) = char_to_dir(c) {
        return if *in_dir == a {
            Some(b)
        } else if *in_dir == b {
            Some(a)
        } else {
            None
        }
    }
    return None
}

fn find_s(maze: &[[char; SIZE]; SIZE]) -> Option<(usize, usize)>{
    for (y,row) in maze.iter().enumerate() {
        for (x, &c) in row.iter().enumerate() {
            if c == 'S' {
                return Some((x,y))
            }
        }
    }
    return None
}

// fn find_start_tubes(maze: &[[char; SIZE]; SIZE]){
//     let (s_x, s_y) = find_s(maze).unwrap();
//     println!("{},{}", s_x, s_y);
//     for dir in [Dir::NORTH, Dir::EAST, Dir::SOUTH, Dir::WEST] {
//         let (o_x, o_y) = dir_to_val(&dir);
//
//         let c = maze[(s_y as i32+o_y) as usize][(s_x as i32+o_x) as usize];
//         let op_dir = oposite_dir(&dir);
//         println!("trying {} with dir {:?}", c, op_dir);
//         next_dir(c, op_dir);
//     }
// }

fn fill_maze(maze: &mut [[char; SIZE]; SIZE], row: usize, line: &str) {
    line.chars().enumerate().for_each(|(i, c)|maze[row][i] = c);
}

fn get_maze_char(maze: &[[char; SIZE]; SIZE], pos: (i32, i32)) -> char {
    maze[pos.1 as usize][pos.0 as usize]
}

fn calc_new_pos(cur_pos: (i32, i32), step: (i32, i32)) -> (i32, i32) {
    (cur_pos.0 + step.0, cur_pos.1 + step.1)
}

fn walk_step(maze: &[[char; SIZE]; SIZE], cur: &mut Step) {
    let cur_c = get_maze_char(maze, cur.pos);
    let out_dir = get_out_dir(cur_c, &cur.in_dir).unwrap();
    let new_pos = calc_new_pos(cur.pos, dir_to_val(&out_dir));
    println!("Going out at: {:?} to {:?}", out_dir, new_pos);
    cur.pos = new_pos;
    cur.in_dir = oposite_dir(&out_dir)
}

fn double_walk(maze: &[[char; SIZE]; SIZE], first: &mut Step, second: &mut Step) {
    walk_step(maze, first);
    walk_step(maze, second);
}

struct Step {
    pos: (i32, i32),
    in_dir: Dir,
}

fn full_walk(maze: &[[char; SIZE]; SIZE], mut first: Step, mut second: Step) {
    let mut counter = 1;
    while first.pos != second.pos {
        double_walk(maze, &mut first, &mut second);
        counter += 1;
    }
    println!("steps: {}", counter);
}

fn main() {
    // let filename = "10small.in";
    // let first_start_tube = Step{pos:(0, 3), in_dir: Dir::NORTH};
    // let second_start_tube = Step{pos:(1, 2), in_dir:Dir::WEST};

    let filename = "10.in";
    let first_start_tube = Step{pos:(88, 86), in_dir: Dir::EAST};
    let second_start_tube = Step{pos:(90, 86), in_dir:Dir::WEST};


    let mut maze: [[char; SIZE]; SIZE] = [['?'; SIZE]; SIZE];
    read_to_string(filename).unwrap().lines().enumerate().for_each(|(i,line)|fill_maze(&mut maze, i, line));
    for row in maze {
        println!("{:?}", row);
    }

    full_walk(&maze, first_start_tube, second_start_tube)
}
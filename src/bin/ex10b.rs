use std::fs::read_to_string;

const SIZE: usize = 140;

#[derive(PartialEq, Debug, Clone)]
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
    // println!("Going out at: {:?} to {:?}", out_dir, new_pos);
    cur.pos = new_pos;
    cur.in_dir = oposite_dir(&out_dir)
}

fn double_walk(maze: &[[char; SIZE]; SIZE], first: &mut Step, second: &mut Step) {
    walk_step(maze, first);
    walk_step(maze, second);
}

#[derive(Clone, Debug)]
struct Step {
    pos: (i32, i32),
    in_dir: Dir,
}

fn full_walk(maze: &[[char; SIZE]; SIZE], maze2: &mut[[char; SIZE]; SIZE], mut first: Step, finish: (i32, i32)) {
    let mut counter = 1;
    while first.pos != finish {
        maze2[first.pos.1 as usize][first.pos.0 as usize] = 'X';
        // match first.in_dir {
        //     Dir::NORTH => maze2[first.pos.1 as usize][first.pos.0 as usize] = 'S',
        //     Dir::EAST => maze2[first.pos.1 as usize][first.pos.0 as usize] = 'W',
        //     Dir::SOUTH => maze2[first.pos.1 as usize][first.pos.0 as usize] = 'N',
        //     Dir::WEST => maze2[first.pos.1 as usize][first.pos.0 as usize] = 'E',
        // }
        walk_step(maze, &mut first);
        counter += 1;
    }
    maze2[first.pos.1 as usize][first.pos.0 as usize] = 'X';
    println!("steps: {}", counter);
}

fn flood_fill_char_start_right(c: char, in_dir: Dir) -> Vec<Dir>{
    match c {
        '|' => {
            return if in_dir == Dir::NORTH {
                vec![Dir::WEST]
            } else {
                vec![Dir::EAST]
            }
        },
        '-' => {
            return if in_dir == Dir::WEST {
                vec![Dir::SOUTH]
            } else {
                vec![Dir::NORTH]
            }
        },
        'L' => {
            return if in_dir == Dir::NORTH {
                vec![Dir::WEST, Dir::SOUTH]
            } else {
                Vec::new()
            }
        },
        'J' => {
            return if in_dir == Dir::WEST {
                vec![Dir::EAST, Dir::SOUTH]
            } else {
                Vec::new()
            }
        },
        '7' => {
            return if in_dir == Dir::SOUTH {
                vec![Dir::EAST, Dir::NORTH]
            } else {
                Vec::new()
            }
        },
        'F' => {
            return if in_dir == Dir::EAST {
                vec![Dir::WEST, Dir::NORTH]
            } else {
                Vec::new()
            }
        },
        _ => {
            Vec::new()
        }
    }
}

fn fill_maze_pos(maze2: &mut[[char; SIZE]; SIZE], pos: (i32, i32)) -> bool{
    let cur_char = maze2[pos.1 as usize][pos.0 as usize];
    if cur_char != 'X' && cur_char != 'O' && cur_char != 'S' {
        maze2[pos.1 as usize][pos.0 as usize] = 'O';
        return true;
    }
    return false
}

fn flood_fill(maze2: &mut[[char; SIZE]; SIZE], pos: (i32, i32)) {
    if fill_maze_pos(maze2, pos) {
        for dir in [Dir::NORTH, Dir::EAST, Dir::SOUTH, Dir::WEST] {
            let val = dir_to_val(&dir);
            let new_pos = calc_new_pos(pos, val);
            flood_fill(maze2, new_pos);
        }
    }
}

fn flood_fill_start(maze: &[[char; SIZE]; SIZE], maze2: &mut[[char; SIZE]; SIZE], cur: Step) {
    let cur_c = get_maze_char(maze, cur.pos);
    for dir in flood_fill_char_start_right(cur_c, cur.in_dir) {
        let fill_start_pos = calc_new_pos(cur.pos, dir_to_val(&dir));
        flood_fill(maze2, fill_start_pos);
    }
}

fn full_walk_with_flood_right(maze: &[[char; SIZE]; SIZE], maze2: &mut[[char; SIZE]; SIZE], mut first: Step, finish: (i32, i32)) {
    let mut counter = 0;
    while first.pos != finish {
        flood_fill_start(maze, maze2, first.clone());
        // if counter % 100 == 0 {
        //     for row in maze2.clone() {
        //         println!("{:?}", row.iter().collect::<String>());
        //     }
        // }

        walk_step(maze, &mut first);


        counter += 1;
    }
    println!("steps: {}", counter);
}

fn main() {
    // let filename = "10small.in";
    // let first_start_tube = Step{pos:(0, 3), in_dir: Dir::NORTH};
    // let second_start_tube = Step{pos:(1, 2), in_dir:Dir::WEST};

    let filename = "10.in";
    // let first_start_tube = Step{pos:(88, 86), in_dir: Dir::EAST};
    // let finish = (90, 86);

    let first_start_tube = Step{pos:(90, 86), in_dir:Dir::WEST};
    let finish = (88, 86);


    let mut maze: [[char; SIZE]; SIZE] = [['?'; SIZE]; SIZE];
    read_to_string(filename).unwrap().lines().enumerate().for_each(|(i,line)|fill_maze(&mut maze, i, line));
    for row in maze {
        println!("{:?}", row);
    }
    let mut maze2 = maze.clone();
    full_walk(&maze, &mut maze2, first_start_tube.clone(), finish);

    full_walk_with_flood_right(&maze, &mut maze2, first_start_tube, finish);

    let mut counter = 0;
    for row in maze2 {
        for c in row {
            if c == 'O' {
                counter +=1;
            }
        }
    }
    for row in maze2 {
        println!("{:?}", row.iter().collect::<String>());
    }

    println!("{}", counter);
}
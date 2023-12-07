use std::fs::read_to_string;

fn parse_line(line: &str) -> u64{
    let (_, values) = line.split_once(':').unwrap();
    let spaces_removed = values.replace(' ', "");
    println!("{}", spaces_removed);
    spaces_removed.parse::<u64>().unwrap()
}

fn is_win(total_time: u64, hold_time: u64, distance: u64) -> bool {
    hold_time * (total_time - hold_time) > distance
}

fn find_a_winner(time: u64, distance: u64) -> u64 {
    let mut try_time = time/2;
    while !is_win(time, try_time, distance) {
        try_time += 1;
    }
    return try_time
}

fn find_left_most_winner_rec(total_time: u64, left_bound: u64, right_bound: u64, distance: u64) -> u64{

    if left_bound+1 == right_bound {
        // TODO: check if this works
        return if is_win(total_time, left_bound, distance) {
            left_bound
        } else {
            right_bound
        };
    }
    let try_time = left_bound + ((right_bound-left_bound) / 2);
    println!("trying {} {} {}", left_bound, try_time, right_bound);
    return if is_win(total_time, try_time, distance) {
        find_left_most_winner_rec(total_time, left_bound, try_time, distance)
    } else {
        find_left_most_winner_rec(total_time, try_time, right_bound, distance)
    }
}

fn find_right_most_winner_rec(total_time: u64, left_bound: u64, right_bound: u64, distance: u64) -> u64{

    if left_bound+1 == right_bound {
        // TODO: check if this works
        return if is_win(total_time, left_bound, distance) {
            left_bound
        } else {
            right_bound
        };
    }
    let try_time = left_bound + ((right_bound-left_bound) / 2);
    println!("trying {} {} {} = {}", left_bound, try_time, right_bound, is_win(total_time, try_time, distance));
    return if is_win(total_time, try_time, distance) {
        find_right_most_winner_rec(total_time, try_time, right_bound, distance)
    } else {
        find_right_most_winner_rec(total_time, left_bound, try_time, distance)
    }
}

// fn find_left_most_winner(total_time: u64, winner_time: u64, distance: u64) {
//     let try_time = (total_time-winner_time) / 2;
//     if is_win(total_time, try_time, distance) {
//         return find_left_most_winner_rec()
//     }
// }

fn calc_win_ways(time: u64, distance: u64) -> u64 {
    println!("{} {}", time, distance);

    let a_winner = find_a_winner(time, distance);
    println!("{}", a_winner);
    let left_winner = find_left_most_winner_rec(time, 0, a_winner, distance);
    println!("Left winner: {}", left_winner);
    let right_winner = find_right_most_winner_rec(time, a_winner, time, distance);
    println!("Right winner: {}", right_winner);
    let result = right_winner - left_winner + 1;
    println!("result: {}", result);
    return result
}

fn main() {
    let filename = "6.in";
    let binding = read_to_string(filename).unwrap();
    let mut lines = binding.lines();
    let time = parse_line(lines.next().unwrap());
    let distance = parse_line(lines.next().unwrap());
    let result = calc_win_ways(time, distance);
    println!("{}", result)
}
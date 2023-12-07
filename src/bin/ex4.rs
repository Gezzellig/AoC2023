use std::fs::read_to_string;

fn process_card(line: &str) -> u32 {
    let (_, useful_part) = line.split_once(": ").unwrap();
    let (win_str, have_str) = useful_part.split_once(" | ").unwrap();
    println!("{}     ha   {}", win_str, have_str);
    let wins: Vec<u32> = win_str.split(' ').map(|x|x.parse::<u32>()).flatten().collect();
    let mut matches = 0;
    for have in have_str.split(' ').map(|x|x.parse::<u32>()).flatten() {
        if wins.contains(&have) {
            matches += 1;
        }
    }
    println!("{}", matches);
    if matches == 0 {
        return 0
    } else {
        let base: u32 = 2;
        return base.pow(matches-1)
    }

}

fn main() {
    let filename = "4small.in";
    let result: u32 = read_to_string(filename).unwrap().lines().map(process_card).sum();
    println!("{}", result)
    // for line in read_to_string(filename).unwrap().lines() {
    //     println!("{}", line);
    //     process_card(line);
    // }
}
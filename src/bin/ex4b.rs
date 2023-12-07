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
    return matches
}

fn main() {
    let filename = "4.in";
    let mut coppies = [1; 211];
    for (i, line) in read_to_string(filename).unwrap().lines().enumerate() {
        // if coppies[i] == 0 {
        //     break
        // }
        let matches = process_card(line);
        for j in i+1..i+(matches as usize)+1 {
            println!("{}", j);
            coppies[j] += coppies[i];
        }
    }
    println!("{}", coppies.iter().sum::<i32>())
    // for line in read_to_string(filename).unwrap().lines() {
    //     println!("{}", line);
    //     process_card(line);
    // }
}
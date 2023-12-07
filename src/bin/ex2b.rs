use std::cmp::max;
use std::fs::read_to_string;
use std::str::FromStr;

struct Show {
    red: u32,
    green: u32,
    blue: u32,
}

impl FromStr for Show {
    type Err = std::num::ParseIntError;
    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut red: u32 = 0;
        let mut green: u32 = 0;
        let mut blue: u32 = 0;

        for segment in line.split(", ") {
            let (amount, color) = segment.split_once(' ').unwrap();
            match color {
                "red" => {
                    red = amount.parse::<u32>().unwrap()
                }
                "green" => {
                    green = amount.parse::<u32>().unwrap()
                }
                "blue" => {
                    blue = amount.parse::<u32>().unwrap()
                }
                _ => {}
            }
        }

        Ok(Show { red, green, blue })
    }
}

fn valid_show(show: &Show, maximum: &Show) -> bool {
    show.red <= maximum.red && show.green <= maximum.green && show.blue <= maximum.blue
}

fn parse_game_line(line: &str) -> (u32, Vec<Show>) {
    let segments: Vec<_> = line.split(": ").collect();
    let game_id = segments[0].split(' ').collect::<Vec<_>>()[1].parse::<u32>().unwrap();
    let shows = segments[1].split("; ").map(Show::from_str).map(|x| x.unwrap()).collect::<Vec<_>>();
    return (game_id, shows)
}

fn go_2b(shows: Vec<Show>) -> u32 {
    let red = shows.iter().map(|x|x.red).max().unwrap();
    let green = shows.iter().map(|x|x.green).max().unwrap();
    let blue = shows.iter().map(|x|x.blue).max().unwrap();
    println!("{} {} {}", red, green, blue);
    return red*green*blue
}

fn process_file(filename: &str) {
    let mut sum = 0;
    for line in read_to_string(filename).unwrap().lines() {
        println!("{}", line);
        let (game_id, shows) = parse_game_line(line);
        sum += go_2b(shows)
    }
    println!("{}", sum)
}

fn main() {
    process_file("2.in")
}
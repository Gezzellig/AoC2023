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
            println!("{}", segment);
            let (amount, color) = segment.split_once(' ').unwrap();
            println!("{}", color);
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
        println!("{} {} {}", red, green, blue);

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

fn valid_game(shows: Vec<Show>, max_values: &Show) -> bool {
    shows.iter().map(|x|valid_show(x, max_values)).all(|x|x)
}

fn process_file(filename: &str, max_values: &Show) {
    let mut sum = 0;
    for line in read_to_string(filename).unwrap().lines() {
        println!("{}", line);
        let (game_id, shows) = parse_game_line(line);
        if valid_game(shows, max_values) {
            sum += game_id
        }
    }
    println!("{}", sum)
}

fn main() {
    let max_values = Show {
        red: 12,
        green: 13,
        blue: 14,
    };
    process_file("2.in", &max_values)
}
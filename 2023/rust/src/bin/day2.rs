use std::fs;

#[derive(Debug)]
struct Set {
    blue: i32,
    red: i32,
    green: i32,
}

#[derive(Debug)]
struct Game {
    id: String,
    sets: Vec<Set>
}

fn main() {
    let data = fs::read_to_string("/home/salakris/Documents/personal/dev/advent-of-code/2023/rust/inputs/day2.prod");

    match data {
        Err(err) => println!("{}", err.to_string()),
        Ok(value) => {
            let lines: Vec<String> = value.lines().map(String::from).collect();
            part_one(&lines);
            part_two(&lines)
        }
    };
}

fn part_one(lines: &Vec<String>) {
    let red_limit = 12;
    let blue_limit = 14;
    let green_limit = 13;

    let mut result = 0;
    let games = parse_game(lines);

    games.iter().for_each(|game|{
        let mut is_valid_game = true;

        for set in &game.sets {
            if set.blue > blue_limit || set.green > green_limit || set.red > red_limit {
                is_valid_game = false;
            }
        }

        if is_valid_game {
            result += game.id.parse::<i32>().expect("We expect aoc input to be correct")
        }
    });

    println!("Part one answer: {}", result);
}

fn part_two(lines: &Vec<String>) {
    let mut result = 0;

    let parsed_games = parse_game(lines);
    parsed_games.iter().for_each(|game| {
        let mut min_blue = 0;
        let mut min_green = 0;
        let mut min_red = 0;

        for set in &game.sets {
            if set.green > min_green {
                min_green = set.green;
            }

            if set.blue > min_blue {
                min_blue = set.blue
            }

            if set.red > min_red {
                min_red = set.red;
            }
        }

        let power = min_red * min_green * min_blue;
        result += power;
    });

    println!("Part two result: {}", result);
}

fn parse_game(lines: &Vec<String>) -> Vec<Game> {
    let mut result: Vec<Game> = Vec::new();

    for line in lines {
        let game_parts: Vec<&str> = line.split(":").collect();
        let game_id = game_parts[0].split(" ").collect::<Vec<&str>>()[1];

        let sets: Vec<&str> = game_parts[1].split(";").collect();

        let mut game: Game = Game{
            id: game_id.to_string(),
            sets: Vec::new()
        };

        let parsed_sets: Vec<Set> = sets.iter().map(|&set| {
            let mut new_set: Set = Set {
                blue: 0,
                green: 0,
                red: 0,
            };

            let cubes: Vec<&str> = set.split(",").collect();
            cubes.iter().for_each(|&x| {
                let c: Vec<&str> = x.split(" ").collect();
                match c[2] {
                    "blue" => new_set.blue = c[1].parse().unwrap(),
                    "green" => new_set.green = c[1].parse().unwrap(),
                    "red" => new_set.red = c[1].parse().unwrap(),
                    _ => {}
                };
            });

            return new_set;
        }).collect();

        game.sets = parsed_sets;
        result.push(game);
    }
    return result;
}


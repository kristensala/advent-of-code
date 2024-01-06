use std::{fs, process::id, i64};
use regex::Regex;

fn main() {
    let data = fs::read_to_string("/home/salakris/Documents/personal/dev/advent-of-code/2023/rust/inputs/day3.prod");

    match data {
        Err(err) => println!("{}", err.to_string()),
        Ok(value) => {
            let lines: Vec<&str> = value.lines().collect();
            part_one(lines);
        }
    };
}

fn part_one(lines: Vec<&str>) {
    let mut result: i64 = 0;

    for (idx, &line) in lines.iter().enumerate() {
        let line_chars: Vec<char> = line.chars().into_iter().collect();

        // build a number
        let mut built_nr: String = String::from("");
        for (char_idx, character) in line_chars.into_iter().enumerate() { 
            if character.is_numeric() {
                built_nr.push(character);
            } 

            if (character.is_numeric() && char_idx == line.len() - 1) || !character.is_numeric() {
                if built_nr.len() > 0 {
                    let nr_position_on_line = line.find(built_nr.as_str())
                        .expect("Number should be on the line");

                    // get the indexes to check on the same line
                    // and on the line above and under
                    // Like this:
                    // ....*****
                    // ....*234*
                    // ....*****

                    let mut start_idx = 0;
                    if nr_position_on_line > 0 {
                        start_idx = nr_position_on_line - 1;
                    }

                    let mut end_idx = nr_position_on_line + built_nr.len() - 1;
                    if end_idx < &line.len() - 1 {
                        end_idx = nr_position_on_line + built_nr.len();
                    }

                    let current_line_sub = &line[start_idx..=end_idx];
                    println!("{}", current_line_sub);

                    if substring_has_special_char(current_line_sub) {
                        result += built_nr.parse::<i64>().expect("Should be a nr");
                        built_nr.clear();
                        continue;
                    }

                    if idx > 0 {
                        let line_above = lines.get(idx - 1);
                        if line_above.is_some() {
                            if substring_has_special_char(&line_above.unwrap()[start_idx..=end_idx]) {
                                result += built_nr.parse::<i64>().expect("Should be a nr");
                                built_nr.clear();
                                continue;
                            }
                        }
                    }

                    let line_under = lines.get(idx + 1);
                    if line_under.is_some() {
                        if substring_has_special_char(&line_under.unwrap()[start_idx..=end_idx]) {
                            result += built_nr.parse::<i64>().expect("Should be a nr");
                            built_nr.clear();
                            continue;
                        }
                    }
                    built_nr.clear();
                }
            }
        }
    }

    println!("Part one result: {}", result);
}

fn substring_has_special_char(substring: &str) -> bool {
    let new_string = substring.replace(".", "");
    let re = Regex::new(r"\W").unwrap();

    return re.is_match(&new_string);
}

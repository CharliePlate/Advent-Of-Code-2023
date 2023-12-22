use regex::Regex;
use std::collections::HashMap;

use crate::utils;

//-> HashMap<String, HashMap<String, u32>>

pub fn part1(limits: &HashMap<String, u32>, input_line: String) -> u32 {
    let parsed = parse_line(input_line);
    for key in limits.keys() {
        if parsed.0.contains_key(key) {
            if parsed.0[key] > limits[key] {
                return 0;
            }
        }
    }
    return parsed.1;
}

fn parse_line(line: String) -> (HashMap<String, u32>, u32) {
    let (_, num) = get_game_num(&line);
    let mut selection_map: HashMap<String, u32> = HashMap::new();
    let num_color = utils::parse_line(line);

    for (num, color) in num_color {
        if selection_map.contains_key(&color) && selection_map[&color] < num {
            selection_map.insert(color, num);
        } else if !selection_map.contains_key(&color) {
            selection_map.insert(color, num);
        }
    }

    (selection_map, num)
}

fn get_game_num(line: &String) -> (usize, u32) {
    let game_num = Regex::new(r"Game (\d+):")
        .unwrap()
        .captures(line.as_str())
        .unwrap()
        .get(1)
        .unwrap();

    let end = game_num.end();
    let num = game_num.as_str().parse::<u32>().unwrap();

    (end, num)
}

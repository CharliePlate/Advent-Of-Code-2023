use std::collections::HashMap;

use crate::utils;
//-> HashMap<String, HashMap<String, u32>>

pub fn part2(input_line: String) -> u32 {
    let parsed = parse_line(input_line);

    parsed.keys().map(|key| parsed[key]).product()
}

fn parse_line(line: String) -> HashMap<String, u32> {
    let mut selection_map: HashMap<String, u32> = HashMap::new();
    let num_color = utils::parse_line(line);

    for (num, color) in num_color {
        if selection_map.contains_key(&color) && selection_map[&color] < num {
            selection_map.insert(color, num);
        } else if !selection_map.contains_key(&color) {
            selection_map.insert(color, num);
        }
    }

    selection_map
}

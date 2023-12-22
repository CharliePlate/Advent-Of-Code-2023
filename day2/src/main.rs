// 1. Parse Input (Probably with Regex) and store in a map.
// Map will look like this
// {
//      gameId: {
//          color: count,
//          color1: count1,
//          ...
//      }
//      gameId1: {
//          ...
//      }
// }
//
// 2. input will be its own map, like
// {
//      red: 12,
//      green: 13,
//      blue: 14
// }
//
// 3. Iterate though HashMap keys, compare the two objects, where if any key of object 1 > any key
//    of object 2, return False

use std::collections::HashMap;
use std::fs;
mod part1;
mod part2;
mod utils;

fn main() {
    let mut limits = HashMap::new();
    limits.insert("red".to_string(), 12);
    limits.insert("green".to_string(), 13);
    limits.insert("blue".to_string(), 14);

    let p1: u32 = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| part1::part1(&limits, l.to_string()))
        .sum();

    let p2: u32 = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| part2::part2(l.to_string()))
        .sum();

    println!("Part 1: {}", p1);
    println!("Part 2: {}", p2);
}

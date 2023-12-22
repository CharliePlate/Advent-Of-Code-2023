use std::fs;
mod part1;
mod part2;

fn main() {
    let part1 = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| part1::part1(l).parse::<u32>().unwrap())
        .sum::<u32>();

    let part2: u32 = fs::read_to_string("./input.txt")
        .unwrap()
        .lines()
        .map(|l| part2::part2(l).parse::<u32>().unwrap())
        .sum::<u32>();

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

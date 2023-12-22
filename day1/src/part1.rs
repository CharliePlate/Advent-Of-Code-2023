pub fn part1(input: &str) -> String {
    let first_word = input.chars().find(|c| c.is_digit(10)).unwrap();
    let second_word = input.chars().rev().find(|c| c.is_digit(10)).unwrap();

    format!("{}{}", first_word, second_word)
}

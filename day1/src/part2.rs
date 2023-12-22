use fancy_regex::Regex;
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
    static ref STRING_TO_NUM: HashMap<&'static str, &'static str> = {
        let mut m = HashMap::new();
        m.insert("one", "1");
        m.insert("two", "2");
        m.insert("three", "3");
        m.insert("four", "4");
        m.insert("five", "5");
        m.insert("six", "6");
        m.insert("seven", "7");
        m.insert("eight", "8");
        m.insert("nine", "9");
        m.insert("zero", "0");
        m
    };
}

pub fn part2(input: &str) -> String {
    let rexp =
        Regex::new(r"(?=(one|two|three|four|five|six|seven|eight|nine|zero|[0-9]))").unwrap();

    println!("{}", input);

    let captures = rexp.captures_iter(input);
    let cap_vec = captures
        .map(|x| {
            x.unwrap()
                .iter()
                .map(|x| x.unwrap().as_str())
                .collect::<String>()
        })
        .map(|c| {
            if c.parse::<u32>().is_ok() {
                c
            } else {
                STRING_TO_NUM.get(&c[..]).unwrap().to_string()
            }
        })
        .collect::<Vec<String>>();

    format!("{}{}", cap_vec[0], cap_vec[cap_vec.len() - 1])
}

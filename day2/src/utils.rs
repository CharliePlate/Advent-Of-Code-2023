use regex::Regex;

pub fn parse_line(line: String) -> Vec<(u32, String)> {
    let picked_regex = Regex::new("(\\d+)\\s(\\w*)").unwrap();
    let picked_cubes = line[line.find(":").unwrap() + 1..].split(";").into_iter();
    let mut result: Vec<(u32, String)> = Vec::new();

    for cube_selection in picked_cubes {
        picked_regex
            .captures_iter(cube_selection)
            .map(|cap| {
                let cube_num = cap.get(1).unwrap().as_str().parse::<u32>().unwrap();
                let cube_color = cap.get(2).unwrap().as_str().to_string();
                (cube_num, cube_color)
            })
            .for_each(|(num, color)| {
                result.push((num, color));
            });
    }

    result
}

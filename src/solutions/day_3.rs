use crate::util::read_file;

pub fn part_one() -> String {
    let contents = read_file(3);
    assert_eq!(get_char_value('a'), 1);
    assert_eq!(get_char_value('A'), 27);
    contents
        .lines()
        .map(|line| {
            let first_part = &line[0..line.len() / 2];
            let second_part = &line[line.len() / 2..line.len()];
            let mut common_char = 'a';
            'outer: for char in first_part.chars() {
                for sec_char in second_part.chars() {
                    if char == sec_char {
                        common_char = char;
                        break 'outer;
                    }
                }
            }
            get_char_value(common_char)
        })
        .sum::<u32>()
        .to_string()
}

pub fn part_two() -> String {
    let content = read_file(3);
    let lines: Vec<&str> = content.lines().collect();
    let mut groups: Vec<Vec<&str>> = vec![];
    let mut group: Vec<&str> = vec![];
    for i in 0..lines.len() {
        group.push(lines[i]);
        if group.len() == 3 {
            groups.push(group);
            group = vec![];
        }
    }
    groups
        .iter()
        .map(|group| {
            let common_char = group[0]
                .chars()
                .filter(|c| group[1].contains(*c))
                .filter(|c| group[2].contains(*c))
                .collect::<Vec<char>>();
            get_char_value(*common_char.first().unwrap())
        })
        .sum::<u32>()
        .to_string()
}

fn get_char_value(character: char) -> u32 {
    let order = character as u32;
    if character.is_uppercase() {
        order - 38
    } else {
        order - 96
    }
}

use crate::util::read_file;

pub fn part_one() -> String {
    let content = read_file(5);
    let parts = content.split("\n\n").collect::<Vec<&str>>();

    // rebuild the start input
    let mut stacks: Vec<Vec<char>> = vec![];
    for _ in 0..9 {
        stacks.push(vec![]);
    }

    for line in parts[0].lines().rev() {
        if line.chars().any(|c| c.is_numeric()) {
            continue;
        }
        let mut column = 1;
        while column < 10 {
            let c = line.chars().nth((column - 1) * 4 + 1).unwrap();
            if c.is_alphabetic() {
                stacks[column - 1].push(c);
            }
            column += 1;
        }
    }

    // process all the changes
    for line in parts[1].lines() {
        let amount = line.split("from").collect::<Vec<&str>>()[0]
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_string())
            .collect::<Vec<String>>()
            .join("")
            .parse::<usize>()
            .unwrap();
        let nums = line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| c.to_string().parse::<usize>().unwrap());
        let from = nums.clone().rev().take(2).collect::<Vec<usize>>()[1] - 1;
        let to = nums.rev().take(1).collect::<Vec<usize>>()[0] - 1;
        let copy = stacks[from].clone();
        let to_shift = copy.iter().rev().take(amount).collect::<Vec<&char>>();
        let old_from_length = stacks[from].len();
        for char in to_shift {
            stacks[to].push(*char);
        }
        for i in 0..amount {
            stacks[from].remove(old_from_length - i - 1);
        }
    }

    // return the top item for each column
    stacks
        .iter()
        .map(|column| column.last().unwrap().to_string())
        .collect::<Vec<String>>()
        .join("")
}

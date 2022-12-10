use std::{collections::HashSet};

use crate::util::read_file;

pub fn part_one() -> usize {
    let actions = &get_actions();
    move_rope(actions, 1)
}

pub fn part_two() -> usize {
    let actions = &get_actions();
    move_rope(actions, 9)
}

type Action = (i16, i16, i16);

fn get_actions() -> Vec<Action> {
    let content = read_file(9);
    content
        .lines()
        .map(|l| {
            let (direction, steps) = l.split_once(' ').unwrap();

            let (x, y) = match direction {
                "L" => (-1, 0),
                "R" => (1, 0),
                "U" => (0, 1),
                "D" => (0, -1),
                _ => unreachable!(),
            };

            (x, y, steps.parse::<i16>().unwrap())
        })
        .collect()

}

fn move_rope(actions: &[Action], knots: usize) -> usize {
    let mut rope: Vec<(i16, i16)> = vec![(0, 0); knots + 1];

    let mut visited: HashSet<(i16, i16)> = HashSet::new();
    visited.insert((0, 0));

    actions.iter().for_each(|(x, y, steps)| {
        (0..*steps).for_each(|_| {
            rope[0].0 += x;
            rope[0].1 += y;

            (1..rope.len()).for_each(|i| {
                let (x_ahead, y_ahead) = rope[i - 1];

                if (x_ahead - rope[i].0).abs() > 1 || (y_ahead - rope[i].1).abs() > 1 {
                    rope[i].0 += (x_ahead - rope[i].0).signum();
                    rope[i].1 += (y_ahead - rope[i].1).signum();
                }
            });

            visited.insert(*rope.last().unwrap());
        });
    });

    visited.len()
}
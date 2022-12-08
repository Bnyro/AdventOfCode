use crate::util::read_file;

pub fn part_one() -> String {
    let input = read_file(8);

    let mut rows: Vec<Vec<usize>> = vec![];
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        rows.push(row);
    }
    // add all edge trees
    let mut visible_count = 2 * rows.len() + 2 * rows[0].len() - 4;

    for column_index in 1..rows.len() - 1 {
        let column = &rows[column_index];
        for row_index in 1..rows[column_index].len() - 1 {
            let tree_height = column[row_index];
            if column[0..row_index].iter().all(|&i| i < tree_height)
                || column[row_index + 1..rows[0].len()]
                    .iter()
                    .all(|i| *i < tree_height)
                || rows[0..column_index]
                    .iter()
                    .map(|row| row[row_index])
                    .all(|i| i < tree_height)
                || rows[column_index + 1..rows.len()]
                    .iter()
                    .map(|row| row[row_index])
                    .all(|i| i < tree_height)
            {
                visible_count = visible_count + 1;
            }
        }
    }
    visible_count.to_string()
}

pub fn part_two() -> String {
    let input = read_file(8);

    let mut rows: Vec<Vec<usize>> = vec![];
    for line in input.lines() {
        let row = line
            .chars()
            .map(|c| c.to_string().parse::<usize>().unwrap())
            .collect::<Vec<usize>>();
        rows.push(row);
    }
    // add all edge trees
    let mut score = 0;

    for column_index in 1..rows.len() - 1 {
        let column = &rows[column_index];
        for row_index in 1..rows[column_index].len() - 1 {
            let tree_height = column[row_index];

            let x_iter_pos = column[row_index + 1..rows[0].len()].iter();
            let x_pos = x_iter_pos.clone().position(|&i| i >= tree_height);

            let x_iter_neg = column[0..row_index].iter().rev();
            let x_neg = x_iter_neg.clone().position(|&i| i >= tree_height);

            let y_iter_pos = rows[column_index + 1..rows.len()]
                .iter()
                .map(|row| row[row_index]);
            let y_pos = y_iter_pos.clone().position(|i| i >= tree_height);

            let y_iter_neg = rows[0..column_index].iter().rev().map(|row| row[row_index]);
            let y_neg = y_iter_neg.clone().position(|i| i >= tree_height);

            let final_score = (x_pos.unwrap_or(x_iter_pos.count() - 1) + 1)
                * (x_neg.unwrap_or(x_iter_neg.count() - 1) + 1)
                * (y_pos.unwrap_or(y_iter_pos.count() - 1) + 1)
                * (y_neg.unwrap_or(y_iter_neg.count() - 1) + 1);
            if final_score > score {
                score = final_score;
            }
        }
    }
    score.to_string()
}

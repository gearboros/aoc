use std::collections::HashMap;
use rayon::prelude::*;

fn main() {
    let input  = include_str!("input");
    let (part1, part2) = do_the_math(&input);
    println!("{}", part1);
    println!("{}", part2);
}

fn do_the_math(p0: &&str) -> (i64, i64) {
    let part1 = part1(p0);
    let part2 = transpose_part2(p0);
    //let part2 = part2(p0);
    (part1, part2)
}

/// first remove last line with ops
/// then transpose and parse numbers.
#[allow(unused)]
fn transpose_part2(input: &str) -> i64 {
    let mut lines: Vec<&str> = input.lines().collect();
    let ops: Vec<&str> = lines.pop().unwrap().split_whitespace().collect();    // Find the maximum line length
    let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

    let mut transposed = Vec::with_capacity(max_len);
    let len = lines.len();

    for col in 0..max_len {
        let mut transposed_line = String::with_capacity(len);

        for line in &lines {
            if let Some(&byte) = line.as_bytes().get(col) {
                let ch = byte as char;
                if !ch.is_whitespace() {
                    transposed_line.push(ch);
                }
            }
        }

        transposed.push(transposed_line);
    }

    let groups: Vec<Vec<String>> = transposed
        .split(|s| s.trim().is_empty())
        .filter(|group| !group.is_empty())
        .map(|group| group.iter().map(|s| s.to_string()).collect())
        .collect();

    groups.par_iter().enumerate().map(|(idx, group)| {
        let op = ops[idx];
        match op {
            "+" => group.iter().filter_map(|op| op.parse::<i64>().ok()).sum::<i64>(),
            "*" => group.iter().filter_map(|op| op.parse::<i64>().ok()).product::<i64>(),
            _ => 0
        }
    }).sum()
}

fn part1(p0: &&str) -> i64 {
    let mut math_grid: HashMap<usize, Vec<String>> = HashMap::new();

    for line in p0.lines() {
        let ops: Vec<String> = line.split_whitespace().map(String::from).collect();
        for (i, op) in ops.iter().enumerate() {
            math_grid.entry(i).or_insert_with(Vec::new).push(op.clone());
        }
    }

    let mut part1 = 0;

    for (_key, ops) in &mut math_grid {
        let op = ops.pop().unwrap();
        part1 = match op.as_str() {
            "+" => part1 + ops.iter().filter_map(|op| op.parse::<i64>().ok()).sum::<i64>(),
            "*" => part1 + ops.iter().filter_map(|op| op.parse::<i64>().ok()).product::<i64>(),
            _ => part1
        };
    }
    part1
}

#[allow(unused)]
fn part2(p0: &&str) -> i64 {
    let lines = p0.lines().collect::<Vec<&str>>();

    let len = lines.iter().map(|l| l.len()).max().unwrap();
    let number_lines = &lines[..lines.len() - 1];
    let op_line = lines.last().unwrap();

    let column_separators = get_column_separators(&lines, len);
    let num_columns = column_separators.len() -1 ;

    let part2: i64 = (0..num_columns)
        .into_par_iter()
        .map(|idx| {
            let start_idx = column_separators[idx];
            let end_idx = column_separators[idx + 1];

            // parse columns in column by going through indexes and pushing digits, ignoring whitespace
            // theoretically the wrong direction, but + and * don't care.
            let values: Vec<String> = (start_idx..end_idx)
                .map(|i| {
                    let val: String = number_lines.iter()
                        .filter_map(|line| line.chars().nth(i))
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    val
                })
                .filter(|val| !val.is_empty())
                .collect();

            let op = op_line.chars().nth(start_idx);
            match op.unwrap() {
                '+' => values.iter().filter_map(|op| op.parse::<i64>().ok()).sum::<i64>(),
                '*' => values.iter().filter_map(|op| op.parse::<i64>().ok()).product::<i64>(),
                _ => 0
            }
        })
        .sum();
    part2
}

/// basically just find all columns that are only whitespace, plus 0 and end
/// that way we have ranges of the bigger columns in which we can parse our numbers
#[allow(unused)]
fn get_column_separators(lines: &Vec<&str>, len: usize) -> Vec<usize> {
    let mut column_separators = Vec::<usize>::new();
    column_separators.push(0);

    for i in 0..len {
        let is_separator = lines.iter().all(|line| line.chars().nth(i) == Some(' '));
        if is_separator {
            column_separators.push(i + 1);
        }
    }

    column_separators.push(len);
    column_separators
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn math_test_input() {
        let input = include_str!("testinput");
        let (part1, part2) = do_the_math(&input);
        assert_eq!(part1, 4277556);
        assert_eq!(part2, 3263827);
    }

    #[test]
    fn math_real_input() {
        let input = include_str!("input");
        let (part1, part2) = do_the_math(&input);
        assert_eq!(part1, 5322004718681);
        assert_eq!(part2, 9876636978528);
    }
}

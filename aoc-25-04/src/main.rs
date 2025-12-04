use std::collections::{HashSet};
use rayon::prelude::*;

fn main() {
    let input = include_str!("input");
    let (part1, part2) = count_paper(input);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn count_paper(p0: &str) -> (usize, usize) {
    let paper_locs: HashSet<(i32, i32)> = p0
        .lines()
        .enumerate()
        .flat_map(|(row, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(col, ch)| {
                    (ch == '@').then_some((row as i32, col as i32))
                })
        })
        .collect();


    let part1 = paper_locs.iter().filter(|&loc| {
        count_adjacent(&paper_locs, loc) < 4
    }).count();

    let mut part2 = 0;
    //let height = p0.lines().count();
    let mut removable_locs = paper_locs.clone();
    loop {
        let removed: Vec<(i32, i32)> = removable_locs
            .par_iter()  
            .filter(|&loc| count_adjacent(&removable_locs, loc) < 4)
            .copied()
            .collect();

        let removed_count = removed.len();
        if removed_count == 0 {
            break;
        }

        part2 += removed_count;

        for loc in &removed {
            removable_locs.remove(loc);
        }

        //display_grid(&removable_locs, height);
    }


    (part1, part2)

}

#[allow(unused)]
fn display_grid(paper_locs: &HashSet<(i32, i32)>, height: usize) {
    for row in 0..height {
        for col in 0..height {
            if paper_locs.contains(&(row as i32, col as i32)) {
                print!("@");
            } else {
                print!(".");
            }
        }
        println!();
    }
}


fn count_adjacent(p0: &HashSet<(i32, i32)>, p1: &(i32, i32)) -> i32 {
    let (x, y) = p1;
    // can ignore out of bounds, since it won't be in our set anyway.
    let directions = [
        (-1, -1), (-1, 0), (-1, 1),
        (0, -1),           (0, 1),
        (1, -1),  (1, 0),  (1, 1),
    ];

    directions
        .iter()
        .filter(|(dx, dy)| p0.contains(&(x + dx, y + dy)))
        .count() as i32
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn count_paper_test() {
        let input = include_str!("testinput");
        assert_eq!(count_paper(input), (13, 43));
    }

    #[test]
    fn count_paper_input() {
        let input = include_str!("input");
        assert_eq!(count_paper(input), (1428, 8936));
    }
}
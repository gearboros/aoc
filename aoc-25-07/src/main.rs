use std::collections::HashSet;

fn main() {
    let input = include_str!("input");
    let (part1, part2) = cross_the_beams(input);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn cross_the_beams(input: &str) -> (usize, usize) {
    let lines: Vec<&str> = input.lines().collect();
    let first = &lines[0];
    let max_idx = first.len() - 1;
    let mut beams = HashSet::new();
    let start_idx = first.find('S').unwrap();

    // part 1, count beams
    beams.insert(start_idx);
    let mut split_count = 0;

    // part 2, count timelines
    // part 2 idea: Start with 1 beam in the middle
    // store the amount of beams at each index, after the first split, this is just two 1s
    // after each split store the amount of beams that hit that split
    // right and left of it and set the split count to 0
    // at the end, the sum of the beam counts for each index, should be the timelines count
    let mut beam_count_per_idx = vec![0; max_idx + 1];
    beam_count_per_idx[start_idx] += 1;

    for line in lines {
        let splits: Vec<usize> = line
            .char_indices()
            .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
            .collect();

        for split_idx in splits {
            if beams.contains(&split_idx) {
                split_count += 1;
                beams.remove(&split_idx);

                let beams_at_idx = beam_count_per_idx[split_idx];

                if split_idx == 0 {
                    beams.insert(1);

                    beam_count_per_idx[1] += beams_at_idx;
                } else if split_idx == max_idx {
                    beams.insert(split_idx - 1);

                    beam_count_per_idx[split_idx - 1] += beams_at_idx;
                } else {
                    beams.insert(split_idx -1);
                    beams.insert(split_idx +1);

                    beam_count_per_idx[split_idx - 1] += beams_at_idx;
                    beam_count_per_idx[split_idx + 1] += beams_at_idx;
                }

                beam_count_per_idx[split_idx] = 0;
            }
        }
    }

    let timelines = beam_count_per_idx.iter().sum();
    (split_count, timelines)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cross_the_beams_test() {
        let input = include_str!("testinput");
        assert_eq!(cross_the_beams(&input), (21, 40));
    }

    #[test]
    fn cross_the_beams_real() {
        let input = include_str!("input");
        assert_eq!(cross_the_beams(&input), (1541, 80158285728929));
    }
}
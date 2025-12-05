fn main() {
    let input = include_str!("input");
    let (part1, part2) = get_fresh_ids(&input);
    println!("part1: {}", part1);
    println!("part2: {}", part2);
}

fn parse_input(input: &str) -> (Vec<(i64, i64)>, Vec<i64>) {
    let mut parts = input.split("\n\n");

    let rules: Vec<(i64, i64)> = parts
        .next()
        .unwrap_or("")
        .lines()
        .filter_map(|line| {
            let mut nums = line.split('-');
            let first = nums.next().unwrap().parse().ok().unwrap();
            let second = nums.next().unwrap().parse().ok().unwrap();
            Some((first, second))
        })
        .collect();

    let ids: Vec<i64> = parts
        .next()
        .unwrap_or("")
        .lines()
        .filter_map(|line| {
            line.trim().parse().ok()
        })
        .collect();

    (rules, ids)
}

fn get_fresh_ids(p0: &&str) -> (usize, i64) {
    let (ranges, ids) = parse_input(p0);

    let part1 = ids.iter().filter(|&&id| {
        ranges.iter().any(|range| id >= range.0 && id <= range.1)
    }).count();

    let combined_ranges = combine_overlapping_ranges(ranges);
    let part2 = combined_ranges.iter()
        .map(|(start, end)| end - start + 1)
        .sum();

    (part1, part2)
}

fn combine_overlapping_ranges(mut ranges: Vec<(i64, i64)>) -> Vec<(i64, i64)> {
    ranges.sort_by_key(|&(start, _)| start);

    let mut combined_ranges = vec![ranges[0]];
    for &(start, end) in &ranges[1..] {
        let current_upper_range = combined_ranges.last_mut().unwrap();
        // overlap -> adjust end to bigger end
        if start <= current_upper_range.1 + 1 {
            current_upper_range.1 = current_upper_range.1.max(end);
        } else {
            combined_ranges.push((start, end));
        }
    }

    combined_ranges
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn fresh_ids_test() {
        let input = include_str!("testinput");
        let (part1, part2) = get_fresh_ids(&input);
        assert_eq!(part1, 3);
        assert_eq!(part2, 14);
    }

    #[test]
    fn fresh_ids_real() {
        let input = include_str!("input");
        let (part1, part2) = get_fresh_ids(&input);
        assert_eq!(part1, 558);
        assert_eq!(part2, 344813017450467);
    }
}

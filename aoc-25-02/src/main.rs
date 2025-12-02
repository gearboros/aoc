use regex::Regex;

fn main() {
    let input = include_str!("input");
    let (part1, part2) = find_invalid(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_invalid(input: &str) -> (u64, u64) {
    let range_pattern = Regex::new(r"(\d+)-(\d+)").unwrap();
    let mut sum = 0;
    let mut sum2 = 0;

    for capture in range_pattern.captures_iter(input) {
        let start: u64 = capture[1].parse().unwrap();
        let end: u64 = capture[2].parse().unwrap();

        for k in start..=end {
            if has_repeating_pattern(k, true) {
                sum += k;
            }
            if has_repeating_pattern(k, false) {
                sum2 += k;
            }
        }
    }

    (sum, sum2)
}

fn has_repeating_pattern(n: u64, only_once: bool) -> bool {
    let s = n.to_string();
    let len = s.len();

    if only_once {
        if len % 2 != 0 {
            return false;
        }
        return is_pattern_repeating(&s, len / 2);
    }

    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            if is_pattern_repeating(&s, pattern_len) {
                return true;
            }
        }
    }

    false
}

fn is_pattern_repeating(s: &str, pattern_len: usize) -> bool {
    let pattern = &s[..pattern_len];
    let repetitions = s.len() / pattern_len;
    let reconstructed = pattern.repeat(repetitions);
    reconstructed == s
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_find_invalid() {
        let input = include_str!("testinput");
        assert_eq!(find_invalid(input), (1227775554, 4174379265));
    }

    #[test]
    fn test_real() {
        let input = include_str!("input");
        assert_eq!(find_invalid(input), (19605500130, 36862281418));
    }
}
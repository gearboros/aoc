use rayon::prelude::*;
use regex::Regex;

fn main() {
    let input = include_str!("input");
    let (part1, part2) = find_invalid(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn find_invalid(input: &str) -> (u64, u64) {
    let range_pattern = Regex::new(r"(\d+)-(\d+)").unwrap();

    range_pattern
        .captures_iter(input)
        .collect::<Vec<_>>()
        .par_iter()
        .map(|capture| {
            let start: u64 = capture[1].parse().unwrap();
            let end: u64 = capture[2].parse().unwrap();

            let mut sum = 0;
            let mut sum2 = 0;

            for k in start..=end {
                if has_repeating_pattern(k, true) {
                    sum += k;
                }
                if has_repeating_pattern(k, false) {
                    sum2 += k;
                }
            }

            (sum, sum2)
        })
        .reduce(|| (0, 0), |a, b| (a.0 + b.0, a.1 + b.1))
}

fn has_repeating_pattern(k: u64, only_once: bool) -> bool {
    let len = k.checked_ilog10().unwrap_or(0) + 1;

    if only_once {
        if len % 2 != 0 {
            return false;
        }
        return is_digit_pattern_repeating(k, len / 2, len);
    }

    for pattern_len in 1..=len / 2 {
        if len % pattern_len == 0 {
            if is_digit_pattern_repeating(k, pattern_len, len) {
                return true;
            }
        }
    }

    false
}

fn is_digit_pattern_repeating(k: u64, pattern_len: u32, len: u32) -> bool {
    let repetitions = len / pattern_len;
    let pattern_len_power_of_10 = 10u64.pow(pattern_len);

    // get pattern by dividing through respective power of 10
    let pattern = k / 10u64.pow(len - pattern_len);

    for i in 1..repetitions {
        let already_checked = len - (i + 1) * pattern_len;

        // ignore already checked digits to the left by dividing through power of 10 + modulo
        let current_segment = (k / 10_u64.pow(already_checked)) % pattern_len_power_of_10;

        if current_segment != pattern {
            return false;
        }
    }

    true
}

#[allow(unused)]
fn has_repeating_pattern_string(n: u64, only_once: bool) -> bool {
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
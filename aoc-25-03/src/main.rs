fn main() {
    let input = include_str!("input");
    let (part1, part2) = calculate_voltage(input);
    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn calculate_voltage(input: &str) -> (i64, i64) {
    let lines = input.lines().collect::<Vec<_>>();

    let voltage = lines.iter().map(|&line| {
        get_more_max_voltage(line, 2)
    }).sum();

    let more_voltage = lines.iter().map(|&line| {
        get_more_max_voltage(line, 12)
    }).sum();

    (voltage, more_voltage)
}

fn get_more_max_voltage(line: &str, length: usize) -> i64 {
    let digits: Vec<i32> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();

    let mut start = 0;
    let mut result = 0i64;

    // always get the biggest digit that can still create an l long number after the collected ones.
    for i in 0..length {
        let end = digits.len().saturating_sub(length - 1 - i);
        let (max_digit, idx) = get_max_digit(&digits, start, end);
        start = idx + 1;
        result = result * 10 + max_digit as i64;
    }
    result
}

fn get_max_digit(digits: &[i32], start: usize, end: usize) -> (i32, usize) {
    let most_digits = &digits[start..end];
    let max_digit = most_digits.iter().max().unwrap();
    let idx = digits
        .iter()
        .enumerate()
        .position(|(idx, d)| d == max_digit && idx >= start)
        .unwrap();
    (*max_digit, idx)
}

#[allow(unused)]
fn get_max_double_digit_voltage(line: &str) -> i32 {
    let digits: Vec<i32> = line.chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d as i32)
        .collect();
    let most_digits = &digits[..digits.len().saturating_sub(1)];

    // biggest digit, that isn't the last.
    let max_digit = most_digits.iter().max().unwrap();
    let idx = digits[..digits.len().saturating_sub(1)]
        .iter()
        .position(|d| d == max_digit)
        .unwrap();

    digits[(idx + 1)..]
        .iter()
        .map(|&d| max_digit * 10 + d)
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_input() {
        let input = include_str!("testinput");
        let voltage = calculate_voltage(input);
        assert_eq!(voltage.0, 357);
        assert_eq!(voltage.1, 3121910778619);
    }

    #[test]
    fn real_input() {
        let input = include_str!("input");
        let voltage = calculate_voltage(input);
        assert_eq!(voltage.0, 17158);
        assert_eq!(voltage.1, 170449335646486);
    }
}

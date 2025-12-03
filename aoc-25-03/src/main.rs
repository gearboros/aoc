fn main() {
    let input = include_str!("input");
    let voltage = calculate_voltage(input);
    println!("Part 1: {}", voltage.0);
    println!("Part 2: {}", voltage.1);
}

fn calculate_voltage(input: &str) -> (i32, i64) {
    let lines = input.lines().collect::<Vec<_>>();

    let voltage = lines.iter().map(|&line| {
        get_max_voltage(line)
    }).sum();

    let more_voltage = lines.iter().map(|&line| {
        get_more_max_voltage(line, 12)
    }).sum();

    (voltage, more_voltage)
}

fn get_more_max_voltage(line: &str, l: usize) -> i64 {
    let digits: Vec<i32> = line.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    let mut start = 0;
    let mut number = String::new();
    // always get the biggest digit that can still create a l long number after the collected ones.
    for i in 0..l {
        let end = digits.len().saturating_sub(11-i);
        let (max_digit, idx) = get_max_digit(&digits, start, end);
        start = idx + 1;
        number.push_str(max_digit.to_string().as_str());
    }
    number.parse::<i64>().unwrap()
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

fn get_max_voltage(p0: &str) -> i32 {
    let digits: Vec<i32> = p0.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
    // biggest digit, that isn't the last.
    let most_digits = &digits[..digits.len().saturating_sub(1)];
    let max_digit = most_digits.iter().max().unwrap();
    let idx = digits[..digits.len().saturating_sub(1)]
        .iter()
        .position(|d| d == max_digit)
        .unwrap();

    let mut max_voltage = 0;

    for i in (idx+1)..digits.len() {
        let voltage = max_digit * 10 + digits[i];
        if voltage > max_voltage {
            max_voltage = voltage;
        }
    }

    max_voltage
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

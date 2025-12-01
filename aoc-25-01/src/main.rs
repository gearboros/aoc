fn main() {
    let input = include_str!("input");
    let password = find_password(input);
    println!("Part 1: {}", password.0);
    println!("Part 2: {}", password.1);
}

fn find_password(p0: &str) -> (i32, i32) {
    let mut curr = 50;
    let lines: Vec<&str> = p0.lines().collect();
    let mut password = 0;
    let mut actual_password = 0;

    for line in lines {
        let dir = line.chars().next().unwrap();
        let value: i32 = line[1..].parse().unwrap();

        // part 2, count crossing before adjusting curr
        // using integer division's cut off behaviour to count zero crossings
        let zero_crossings = match dir {
            'R' => {
                (curr + value) / 100
            }
            'L' => {
                if curr == 0 {
                    // starting at zero, every 100 counts as a crossing
                    value / 100
                } else if value >= curr {
                    // not zero and value > curr => one crossing plus any additional full 100s
                    1 + (value - curr) / 100
                } else {
                    0
                }
            }
            _ => panic!("Invalid direction"),
        };

        actual_password += zero_crossings;

        match dir {
            'R' => {
                curr = (curr + value).rem_euclid(100);
            }
            'L' => {
                curr = (curr - value).rem_euclid(100);
            }
            _ => {}
        }

        if curr == 0 {
            password += 1;
        }
    }

    (password, actual_password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testinput_1() {
        let input = include_str!("testinput");
        assert_eq!(find_password(input).0, 3);
        assert_eq!(find_password(input).1, 6);
    }

    #[test]
    fn input_1() {
        let input = include_str!("input");
        assert_eq!(find_password(input).0, 1007);
        assert_eq!(find_password(input).1, 5820);
    }
}

pub fn run() -> u32 {
    part1()
}

fn part1() -> u32 {
    let input = include_str!("input1.txt");

    let mut sum = 0;

    for line in input.lines() {
        let mut first_digit = None;
        let mut last_digit = None;

        for c in line.chars() {
            if c.is_digit(10) {
                last_digit = Some(c.to_digit(10).unwrap_or(0));
                if first_digit.is_none() {
                    first_digit = last_digit;
                }
            }
        }

        if let (Some(first), Some(last)) = (first_digit, last_digit) {
            sum += first * 10 + last;
        }
    }

    sum
}

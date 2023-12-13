fn process_line(line: &str) -> u32 {
    let mut first_digit: u32 = 0;
    let mut last_digit: u32 = 0;

    // Find the first digit
    if let Some(first) = line.chars().find(|c| c.is_ascii_digit()) {
        first_digit = first.to_digit(10).unwrap();
    }

    // Find the last digit
    if let Some(last) = line.chars().rev().find(|c| c.is_ascii_digit()) {
        last_digit = last.to_digit(10).unwrap();
    }

    return first_digit * 10 + last_digit;
}

pub fn solution() -> u32 {
    let input_str = include_str!("./data/input.txt");
    return input_str.lines().map(process_line).sum();
}

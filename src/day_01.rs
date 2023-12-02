use std::fs;
use std::io::{BufRead, BufReader};

fn extract_digits(line: &str) -> Option<u64> {
    let digits: Vec<_> = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).expect("Could not parse digit"))
        .collect();

    if digits.is_empty() {
        return None;
    }

    let mut num = 0;
    num += (*digits.first().unwrap() as u64) * 10;
    num += *digits.last().unwrap() as u64;
    Some(num)
}

/// Parse a lowercase English digit: "one", "two", ... "nine"
fn parse_digit(input: &str) -> Option<u64> {
    match input {
        "one" => Some(1),
        "two" => Some(2),
        "three" => Some(3),
        "four" => Some(4),
        "five" => Some(5),
        "six" => Some(6),
        "seven" => Some(7),
        "eight" => Some(8),
        "nine" => Some(9),
        _ => None,
    }
}

fn extract_more_digits(line: &str) -> Option<u64> {
    if line.is_empty() {
        return None;
    }

    // first try to match a digit
    //line.chars().into_iter().fir

    None
}

pub fn day_01_a(path_name: &str) -> u64 {
    let reader = BufReader::new(fs::File::open(path_name).unwrap());

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            extract_digits(&line).expect("No digits found in line")
        })
        .sum()
}

pub fn day_01_b(path_name: &str) -> u64 {
    let reader = BufReader::new(fs::File::open(path_name).unwrap());

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            extract_more_digits(&line).expect("No digits found in line")
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_digits() {
        assert_eq!(extract_digits("cats"), None);
        assert_eq!(extract_digits("1abc2"), Some(12));
        assert_eq!(extract_digits("pqr3stu8vwx"), Some(38));
        assert_eq!(extract_digits("a1b2c3d4e5f"), Some(15));
        assert_eq!(extract_digits("treb7uchet"), Some(77));
    }

    #[test]
    fn test_extract_more_digits() {
        assert_eq!(extract_more_digits("cats"), None);
        assert_eq!(extract_more_digits("two1nine"), Some(29));
        assert_eq!(extract_more_digits("eightwothree"), Some(83));
        assert_eq!(extract_more_digits("abcone2threexyz"), Some(13));
        assert_eq!(extract_more_digits("xtwone3four"), Some(24));
        assert_eq!(extract_more_digits("4nineeightseven2"), Some(42));
        assert_eq!(extract_more_digits("zoneight234"), Some(14));
        assert_eq!(extract_more_digits("7pqrstsixteen"), Some(76));
    }

    #[test]
    fn test_parse_digit() {
        assert_eq!(parse_digit("one"), Some(1));
        assert_eq!(parse_digit("five"), Some(5));
        assert_eq!(parse_digit("sixteen"), None);
    }

    #[test]
    fn test_part1() {
        let result = day_01_a("inputs/day_01_test.txt");
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2() {
        let result = day_01_b("inputs/day_01_test.txt");
        assert_eq!(result, 281);
    }
}

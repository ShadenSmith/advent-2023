use nom::{
    branch::alt, bytes::complete::tag, character::complete::anychar, combinator::map_res,
    error::ErrorKind, IResult,
};
use std::fs;
use std::io::{BufRead, BufReader};

fn digits_to_num(digits: &[u64]) -> Option<u64> {
    if digits.is_empty() {
        return None;
    }
    let mut num = 0;
    num += (*digits.first().unwrap()) * 10;
    num += *digits.last().unwrap();
    Some(num)
}

//
// Part 1
//
fn extract_digits(line: &str) -> Option<u64> {
    let digits: Vec<_> = line
        .chars()
        .filter(|c| c.is_numeric())
        .map(|c| c.to_digit(10).unwrap() as u64)
        .collect();

    digits_to_num(&digits)
}

pub fn day_01_a(path_name: &str) -> u64 {
    let reader = BufReader::new(fs::File::open(path_name).expect("Could not open input"));

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            extract_digits(&line).expect("No digits found in line")
        })
        .sum()
}

//
// Part 2
//

fn parse_digit_word(input: &str) -> IResult<&str, u64> {
    // TODO: Avoid allocation from doing char.to_string()
    let digit = map_res(anychar, |s: char| s.to_string().parse::<u64>());
    // TODO: less ugliness
    let one = map_res(tag("one"), |_s| Ok::<u64, ErrorKind>(1));
    let two = map_res(tag("two"), |_s| Ok::<u64, ErrorKind>(2));
    let three = map_res(tag("three"), |_s| Ok::<u64, ErrorKind>(3));
    let four = map_res(tag("four"), |_s| Ok::<u64, ErrorKind>(4));
    let five = map_res(tag("five"), |_s| Ok::<u64, ErrorKind>(5));
    let six = map_res(tag("six"), |_s| Ok::<u64, ErrorKind>(6));
    let seven = map_res(tag("seven"), |_s| Ok::<u64, ErrorKind>(7));
    let eight = map_res(tag("eight"), |_s| Ok::<u64, ErrorKind>(8));
    let nine = map_res(tag("nine"), |_s| Ok::<u64, ErrorKind>(9));
    alt((digit, one, two, three, four, five, six, seven, eight, nine))(input)
}

fn extract_more_digits(line: &str) -> Option<u64> {
    let mut digits = Vec::new();
    let mut line = line;

    while !line.is_empty() {
        if let Ok(result) = parse_digit_word(line) {
            line = result.0;
            digits.push(result.1);
        } else {
            // move to next char
            // TODO: move this logic to a nom parser
            line = &line[1..];
        }
    }

    digits_to_num(&digits)
}

pub fn day_01_b(path_name: &str) -> u64 {
    let reader = BufReader::new(fs::File::open(path_name).expect("Could not open input"));

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
    fn test_parse_digit_word() {
        assert_eq!(parse_digit_word("1c"), Ok(("c", 1)));
        assert_eq!(parse_digit_word("12"), Ok(("2", 1)));
        assert_eq!(parse_digit_word("one1"), Ok(("1", 1)));
        assert_eq!(parse_digit_word("fivex"), Ok(("x", 5)));
        assert_eq!(parse_digit_word("sixteen"), Ok(("teen", 6)));
        assert!(parse_digit_word("teenfive").is_err());
    }

    #[test]
    fn test_part1() {
        let result = day_01_a("inputs/day_01_test_a.txt");
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part2() {
        let result = day_01_b("inputs/day_01_test_b.txt");
        assert_eq!(result, 281);
    }
}

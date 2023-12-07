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

fn extract_more_digits(line: &str) -> Option<u64> {
    let word_names = [
        "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let mut input = line.to_string();

    // pad each word with a repeated

    // replace eight with eightt to catch "eighttwo" and "eightthree" as two tokens
    input = input.replace("one", "oone");
    input = input.replace("two", "ttwo");
    input = input.replace("three", "tthree");
    input = input.replace("eight", "eightt");

    // replace instances of "one" with "1", etc.
    for (i, word) in word_names.iter().enumerate() {
        input = input.replace(word, &(i + 1).to_string());
    }

    extract_digits(&input)
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
        assert_eq!(extract_more_digits("two1nine"), Some(29));
        assert_eq!(extract_more_digits("eightwothree"), Some(83));
        assert_eq!(extract_more_digits("abcone2threexyz"), Some(13));
        assert_eq!(extract_more_digits("xtwone3four"), Some(24));
        assert_eq!(extract_more_digits("4nineeightseven2"), Some(42));
        assert_eq!(extract_more_digits("zoneight234"), Some(14));
        assert_eq!(extract_more_digits("7pqrstsixteen"), Some(76));
        assert_eq!(extract_more_digits("cats"), None);
        assert_eq!(extract_more_digits("338"), Some(38));
        assert_eq!(extract_more_digits("seven88"), Some(78));
        assert_eq!(extract_more_digits("qjgr1"), Some(11));
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

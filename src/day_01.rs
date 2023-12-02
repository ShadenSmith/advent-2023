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
    fn test_day1() {
        let result = day_01_a("inputs/day_01_test.txt");
        assert_eq!(result, 142);
    }
}

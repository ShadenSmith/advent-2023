use std::default::Default;

use std::fs;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
struct ParseRangeError;

// TODO:  add an IoError to drop the remaining unwraps?
#[derive(Debug, PartialEq, Eq)]
enum ParseAlmanacError {
    SeedsError,
    RangeError(ParseRangeError),
}

#[derive(Debug, PartialEq, Eq)]
struct Range {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl Range {
    pub fn remap(&self, index: usize) -> Option<usize> {
        if index < self.source_start {
            return None;
        }

        let offset = index - self.source_start;

        if offset >= self.length {
            return None;
        }

        Some(self.destination_start + offset)
    }
}

impl FromStr for Range {
    type Err = ParseRangeError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elems: Vec<&str> = s.split_whitespace().collect();
        if elems.len() != 3 {
            return Err(ParseRangeError);
        }

        let destination_start = elems[0].parse().map_err(|_| ParseRangeError)?;
        let source_start = elems[1].parse().map_err(|_| ParseRangeError)?;
        let length = elems[2].parse().map_err(|_| ParseRangeError)?;

        Ok(Self {
            destination_start,
            source_start,
            length,
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct RangeMap {
    ranges: Vec<Range>,
}

impl Default for RangeMap {
    fn default() -> Self {
        Self { ranges: Vec::new() }
    }
}
impl RangeMap {
    pub fn remap(&self, source: usize) -> usize {
        for range in self.ranges.iter() {
            if let Some(dest) = range.remap(source) {
                return dest;
            }
        }

        return source;
    }

    pub fn add_range(&mut self, range: Range) {
        self.ranges.push(range);
    }

    pub fn new() -> Self {
        Self::default()
    }
}

struct Almanac {
    seeds: Vec<usize>,
    seed_to_soil: RangeMap,
    soil_to_fertilizer: RangeMap,
    fertilizer_to_water: RangeMap,
    water_to_light: RangeMap,
    light_to_temperature: RangeMap,
    temperature_to_humidity: RangeMap,
    humidity_to_location: RangeMap,
}

impl Almanac {
    fn parse_seeds(line: &str) -> Result<Vec<usize>, ParseAlmanacError> {
        if !line.starts_with("seeds: ") {
            return Err(ParseAlmanacError::SeedsError);
        }

        let (header, rest) = line.split_once(" ").unwrap();

        let seeds: Vec<usize> = rest
            .split_whitespace()
            .into_iter()
            .map(|v| v.parse().unwrap())
            .collect();

        Ok(seeds)
    }

    fn parse_map(reader: &mut BufReader<fs::File>) -> Result<RangeMap, ParseAlmanacError> {
        let mut line_buf = String::new();
        // name
        reader.read_line(&mut line_buf).unwrap();

        let mut ranges = Vec::new();

        loop {
            line_buf.clear();
            reader.read_line(&mut line_buf).unwrap();

            if let Ok(range) = line_buf.parse() {
                ranges.push(range);
            } else {
                break;
            }
        }

        Ok(RangeMap { ranges })
    }

    pub fn from_path(path: &str) -> Result<Self, ParseAlmanacError> {
        let mut reader = BufReader::new(fs::File::open(path).expect("Could not open input"));

        let mut line_buf = String::new();

        reader.read_line(&mut line_buf).unwrap();

        let seeds = Almanac::parse_seeds(&line_buf)?;
        line_buf.clear();
        reader.read_line(&mut line_buf).unwrap();
        line_buf.clear();

        let seed_to_soil = Almanac::parse_map(&mut reader)?;
        let soil_to_fertilizer = Almanac::parse_map(&mut reader)?;
        let fertilizer_to_water = Almanac::parse_map(&mut reader)?;
        let water_to_light = Almanac::parse_map(&mut reader)?;
        let light_to_temperature = Almanac::parse_map(&mut reader)?;
        let temperature_to_humidity = Almanac::parse_map(&mut reader)?;
        let humidity_to_location = Almanac::parse_map(&mut reader)?;

        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temperature,
            temperature_to_humidity,
            humidity_to_location,
        })
    }
}

pub fn day_05_a(path_name: &str) -> u64 {
    let almanac = Almanac::from_path(path_name);
    todo!()
}

pub fn day_05_b(path_name: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        let result = day_05_a("inputs/day_05_test_a.txt");
        assert_eq!(result, 35);
    }

    #[test]
    fn test_range_remap() {
        let range = Range {
            destination_start: 50,
            source_start: 98,
            length: 2,
        };

        assert_eq!(range.remap(98), Some(50));
        assert_eq!(range.remap(99), Some(51));

        assert_eq!(range.remap(97), None);
        assert_eq!(range.remap(100), None);
        assert_eq!(range.remap(1), None);
    }

    #[test]
    fn test_range_parse() {
        let range: Range = "50 98 2".parse().unwrap();

        assert_eq!(range.destination_start, 50);
        assert_eq!(range.source_start, 98);
        assert_eq!(range.length, 2);

        let bad_len = "50 98".parse::<Range>();
        assert!(bad_len.is_err());

        let bad_int = "50 hi 2".parse::<Range>();
        assert!(bad_int.is_err());
    }

    #[test]
    fn test_map_empty() {
        let map = RangeMap::default();
        assert_eq!(map.remap(10), 10);
    }

    #[test]
    fn test_map_example() {
        let map = RangeMap {
            ranges: vec![
                Range {
                    destination_start: 50,
                    source_start: 98,
                    length: 2,
                },
                Range {
                    destination_start: 52,
                    source_start: 50,
                    length: 48,
                },
            ],
        };

        assert_eq!(map.remap(98), 50);
        assert_eq!(map.remap(99), 51);

        assert_eq!(map.remap(50), 52);
        assert_eq!(map.remap(53), 55);
        assert_eq!(map.remap(97), 99);

        assert_eq!(map.remap(10), 10);
    }

    #[test]
    fn test_almanac_parse() {
        let alm = Almanac::from_path("inputs/day_05_test_a.txt").unwrap();
        assert_eq!(alm.seeds, vec![79, 14, 55, 13]);

        assert_eq!(
            alm.water_to_light,
            RangeMap {
                ranges: vec![
                    Range {
                        destination_start: 88,
                        source_start: 18,
                        length: 7
                    },
                    Range {
                        destination_start: 18,
                        source_start: 25,
                        length: 70
                    },
                ],
            }
        );
    }
}

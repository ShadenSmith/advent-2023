use std::default::Default;
use std::fs;
use std::io::{BufRead, BufReader};

struct Range {
    destination_start: usize,
    source_start: usize,
    length: usize,
}

impl Range {
    pub fn contains(&self, source: usize) -> Option<usize> {
        todo!()
    }
}

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
            if let Some(dest) = range.contains(source) {
                return dest;
            }
        }

        return source;
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
    pub fn from_path(path: &str) -> Self {
        todo!()
    }
}

pub fn day_05_a(path_name: &str) -> u64 {
    let reader = BufReader::new(fs::File::open(path_name).expect("Could not open input"));

    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();

            0
        })
        .sum()
}

pub fn day_05_b(path_name: &str) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {}

    #[test]
    fn test_part2() {}

    #[test]
    fn test_range_map_empty() {
        let map = RangeMap::default();
        assert_eq!(map.remap(10), 10);
    }

    #[test]
    fn test_range_map_simple() {
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
}

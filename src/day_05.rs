use std::default::Default;
use std::fs;
use std::io::{BufRead, BufReader};

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
    fn test_range_map_empty() {
        let map = RangeMap::default();
        assert_eq!(map.remap(10), 10);
    }

    #[test]
    fn test_range_map_example() {
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

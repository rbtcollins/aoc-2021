use std::ops::Bound;

use btree_range_map::RangeSet;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

peg::parser! {
  grammar sensor_parser() for str {
    rule number() -> i64
      = n:$("-"? ['0'..='9']+) {? n.parse().or(Err("i64")) }

    pub rule newline()
      = ['\r' | '\n' ]*<1,2>

    pub rule sensor() -> (i64, i64, i64, i64)
      = "Sensor at x=" sx:number() ", y=" sy:number() ": closest beacon is at x=" bx:number() ", y=" by:number() {(sx,sy,bx,by)
    }

    pub rule sensors() -> Vec<(i64, i64, i64, i64)>
     = i:sensor() ** newline() newline()? {i}
  }
}

pub fn generate(input: &str) -> Vec<(i64, i64, i64, i64)> {
    sensor_parser::sensors(input).unwrap()
}

fn manhattan(x: i64, y: i64, a: i64, b: i64) -> u64 {
    x.abs_diff(a) + y.abs_diff(b)
}

// 1 -> 1
// 2 -> 3
// 3 -> 5
fn odds(n: i64) -> i64 {
    n * 2 - 1
}

pub fn part_1(input: &[(i64, i64, i64, i64)]) -> usize {
    part_1_helper(input, 2000000)
}

pub fn part_2(input: &[(i64, i64, i64, i64)]) -> usize {
    part_2_helper(input, 4000000)
}

pub fn part_1_helper(input: &[(i64, i64, i64, i64)], row: i64) -> usize {
    let mut exclusions: RangeSet<i64> = RangeSet::new();
    let mut ranges: RangeSet<i64> = RangeSet::new();
    for (sx, sy, bx, by) in input.iter().cloned() {
        // is the sensor in the row?
        if sy == row {
            exclusions.insert(sx);
        }
        // a known beacon also can't be an unknown beacon
        if by == row {
            exclusions.insert(bx);
        }
        let beacon_distance = manhattan(sx, sy, bx, by);
        // can it affect row?
        let row_distance = sy.abs_diff(row);
        if row_distance > beacon_distance {
            continue;
        }

        // beacon 1 away, same row as sensor => want 2nd row of triangle
        // beacon 1 away, one row away from sensor, want 1 row of triangle
        let row_impact = 1 + beacon_distance - row_distance;
        let width = odds(row_impact as i64);
        let start = sx - (width >> 1);
        ranges.insert(start..(start + width))
    }
    for range in exclusions {
        ranges.remove(range);
    }
    ranges.iter().map(|r| r.len() as usize).sum()
}

pub fn part_2_helper(input: &[(i64, i64, i64, i64)], max_coordinate: i64) -> usize {
    (0..=max_coordinate)
        .into_par_iter()
        .map(|y| {
            let mut ranges: RangeSet<i64> = RangeSet::new();
            for (sx, sy, bx, by) in input.iter().cloned() {
                // sensors and beacons are both in the manhattan distance so not handled specially
                let beacon_distance = manhattan(sx, sy, bx, by);
                // can it affect row?
                let row_distance = sy.abs_diff(y);
                if row_distance > beacon_distance {
                    continue;
                }

                // beacon 1 away, same row as sensor => want 2nd row of triangle
                // beacon 1 away, one row away from sensor, want 1 row of triangle
                let row_impact = 1 + beacon_distance - row_distance;
                let width = odds(row_impact as i64);
                let start = sx - (width >> 1);
                ranges.insert(start..(start + width))
                // could check here for early exit perhaps
            }
            for range in ranges.iter() {
                match range.start {
                    Bound::Unbounded => (),
                    Bound::Included(start) => {
                        if start > 0 {
                            // start - 1 is the x coordinate of a beacon
                            return (y + 4000000 * (start - 1)) as usize;
                        }
                    }
                    Bound::Excluded(start) => {
                        if start >= 0 {
                            // start is the x coordinate of a beacon
                            return (y + 4000000 * start) as usize;
                        }
                    }
                }
                match range.end {
                    Bound::Unbounded => (),
                    Bound::Included(end) => {
                        if end < max_coordinate {
                            // end + 1 is the x coordinate of a beacon
                            return (y + 4000000 * (end + 1)) as usize;
                        }
                    }
                    Bound::Excluded(end) => {
                        if end <= max_coordinate {
                            // end is the x coordinate of a beacon
                            return (y + 4000000 * end) as usize;
                        }
                    }
                }
            }
            0
        })
        .sum()
}

#[cfg(test)]
mod tests {

    const SAMPLE: &str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn parse() {
        assert_eq!(
            super::sensor_parser::sensors(SAMPLE),
            Ok(vec![
                (2, 18, -2, 15),
                (9, 16, 10, 16),
                (13, 2, 15, 3),
                (12, 14, 10, 16),
                (10, 20, 10, 16),
                (14, 17, 10, 16),
                (8, 7, 2, 10),
                (2, 0, 2, 10),
                (0, 11, 2, 10),
                (20, 14, 25, 17),
                (17, 20, 21, 22),
                (16, 7, 15, 3),
                (14, 3, 15, 3),
                (20, 1, 15, 3)
            ])
        );
    }

    #[test]
    fn part_1() {
        let input = super::generate(SAMPLE);
        assert_eq!(26, super::part_1_helper(&input, 10));
    }

    #[test]
    fn part_2() {
        let input = super::generate(SAMPLE);
        assert_eq!(56000011, super::part_2_helper(&input, 20));
    }
}

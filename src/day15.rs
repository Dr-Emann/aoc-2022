pub(crate) use super::unimplemented_part as part_2;
use std::cmp;
use std::collections::BTreeSet;
use std::ops::Range;

type Point = (i32, i32);

#[derive(Debug, Copy, Clone)]
pub struct SensorBeacon {
    sensor: Point,
    beacon: Point,
    dist: u32,
}

impl SensorBeacon {
    fn x_range_at_y(self, y: i32) -> Range<i32> {
        let y_dist = self.sensor.1.abs_diff(y);

        let width = (self.dist * 2 + 1).saturating_sub(2 * y_dist);
        let center = self.sensor.0;

        let start = center.sub_unsigned(width / 2);
        start..start.add_unsigned(width)
    }
}

fn parse_pos(pos: &str) -> Point {
    let (x, y) = pos.split_once(", ").unwrap();
    let x = x.strip_prefix("x=").unwrap();
    let y = y.strip_prefix("y=").unwrap();

    (x.parse().unwrap(), y.parse().unwrap())
}

pub fn generator(s: &str) -> Vec<SensorBeacon> {
    let mut result = Vec::with_capacity(1024);
    for line in s.lines() {
        let line = line.strip_prefix("Sensor at ").unwrap();
        let (sensor, beacon) = line.split_once(": closest beacon is at ").unwrap();
        let sensor = parse_pos(sensor);
        let beacon = parse_pos(beacon);
        result.push(SensorBeacon {
            sensor,
            beacon,
            dist: dist(sensor, beacon),
        });
    }
    result
}

fn count_non_beacons_at_y(items: &[SensorBeacon], y: i32) -> u32 {
    let mut known_beacons = BTreeSet::new();
    let mut ranges: Vec<_> = items
        .iter()
        .filter_map(|sb| {
            if sb.beacon.1 == y {
                known_beacons.insert(sb.beacon.0);
            }
            let range = sb.x_range_at_y(y);
            (!range.is_empty()).then_some(range)
        })
        .collect();
    ranges.sort_unstable_by_key(|range| range.start);
    let mut i = 1;
    let mut j = 0;
    while i < ranges.len() {
        if ranges[j].end >= ranges[i].start {
            ranges[j].end = cmp::max(ranges[i].end, ranges[j].end);
        } else {
            j += 1;
            ranges[j] = ranges[i].clone();
        }
        i += 1;
    }
    ranges.truncate(j + 1);
    let res: u32 = ranges
        .iter()
        .map(|range| range.end.abs_diff(range.start))
        .sum();
    res - known_beacons.len() as u32
}

pub fn part_1(items: &[SensorBeacon]) -> u32 {
    count_non_beacons_at_y(items, 2000000)
}

fn dist(lhs: Point, rhs: Point) -> u32 {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1)
}

trait SignedUnsigned {
    type Unsigned;
    fn sub_unsigned(self, other: Self::Unsigned) -> Self;
    fn add_unsigned(self, other: Self::Unsigned) -> Self;
}

impl SignedUnsigned for i32 {
    type Unsigned = u32;
    #[inline(always)]
    fn sub_unsigned(self, other: u32) -> Self {
        if cfg!(debug_assertions) {
            self.checked_sub_unsigned(other).unwrap()
        } else {
            self.wrapping_sub_unsigned(other)
        }
    }

    #[inline(always)]
    fn add_unsigned(self, other: Self::Unsigned) -> Self {
        if cfg!(debug_assertions) {
            self.checked_add_unsigned(other).unwrap()
        } else {
            self.wrapping_add_unsigned(other)
        }
    }
}

super::day_test! {part_1 == 4748135}

#[test]
fn test_demo_1() {
    let input = super::day_test!(@demo_input);
    let input = generator(&input);
    assert_eq!(count_non_beacons_at_y(&input, 10), 26);
}

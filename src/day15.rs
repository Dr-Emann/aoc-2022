pub(crate) use super::unimplemented_part as part_2;

type Point = (i32, i32);

#[derive(Debug, Copy, Clone)]
pub struct SensorBeacon {
    sensor: Point,
    beacon: Point,
    dist: u32,
}

impl SensorBeacon {
    fn is_within_range(self, other: Point) -> bool {
        self.dist() >= dist(self.sensor, other)
    }

    pub fn dist(self) -> u32 {
        self.dist
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
    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = 0;
    let mut max_y = 0;

    for item in items {
        min_x = min_x.min(item.sensor.0.checked_sub_unsigned(item.dist()).unwrap());
        max_x = max_x.max(item.sensor.0.checked_add_unsigned(item.dist()).unwrap());
        min_y = min_y.min(item.sensor.1.checked_sub_unsigned(item.dist()).unwrap());
        max_y = max_y.max(item.sensor.1.checked_add_unsigned(item.dist()).unwrap());
    }

    let mut count = 0;
    let mut can_influence: Vec<_> = items
        .iter()
        .copied()
        .filter(|sb| {
            (sb.sensor.1 <= y && sb.sensor.1.checked_add_unsigned(sb.dist()).unwrap() > y)
                || (sb.sensor.1 >= y && sb.sensor.1.checked_sub_unsigned(sb.dist()).unwrap() < y)
        })
        .collect();
    for x in min_x..=max_x {
        for i in 0..can_influence.len() {
            let item = can_influence[i];
            if (x, y) == item.beacon {
                continue;
            }
            if item.is_within_range((x, y)) {
                // Move the sensor to the front, we're likely to find it again
                can_influence.swap(0, i);
                count += 1;
                break;
            }
        }
    }
    count
}

pub fn part_1(items: &[SensorBeacon]) -> u32 {
    count_non_beacons_at_y(items, 2000000)
}

fn dist(lhs: Point, rhs: Point) -> u32 {
    lhs.0.abs_diff(rhs.0) + lhs.1.abs_diff(rhs.1)
}

super::day_test! {part_1 == 4748135}

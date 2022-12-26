pub(crate) use super::unimplemented_part as part_2;
use std::fmt::Write;
use std::str::FromStr;
use std::{fmt, mem};

pub fn generator(s: &str) -> Vec<&str> {
    s.lines().collect()
}

pub fn part_1(items: &[&str]) -> SnafuNum {
    let mut total = SnafuNum(0);
    for item in items {
        let snafu: SnafuNum = item.parse().unwrap();
        total.0 += snafu.0;
    }
    total
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct SnafuNum(u64);

impl FromStr for SnafuNum {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut res = 0i64;
        for b in s.bytes() {
            res *= 5;
            let digit = match b {
                b'2' => 2,
                b'1' => 1,
                b'0' => 0,
                b'-' => -1,
                b'=' => -2,
                _ => return Err("invalid digit"),
            };
            res += digit;
        }
        if res < 0 {
            return Err("negative value");
        }

        Ok(SnafuNum(res as u64))
    }
}

impl fmt::Display for SnafuNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut n = self.0;

        if n == 0 {
            return f.write_char('0');
        }
        let mut result = Vec::new();
        let mut carry = false;
        while n > 0 || carry {
            let rem = n % 5;
            n /= 5;
            let ch = match rem + (u64::from(mem::take(&mut carry))) {
                0 => b'0',
                1 => b'1',
                2 => b'2',
                3 => {
                    carry = true;
                    b'='
                }
                4 => {
                    carry = true;
                    b'-'
                }
                5 => {
                    carry = true;
                    b'0'
                }
                _ => unreachable!(),
            };
            result.push(ch);
        }

        for ch in result.into_iter().rev() {
            f.write_char(ch as char)?;
        }
        Ok(())
    }
}

super::day_test! {demo_1 == SnafuNum::from_str("2=-1=0").unwrap()}
super::day_test! {part_1 == SnafuNum::from_str("2-==10===-12=2-1=-=0").unwrap()}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLES: &[(u64, &str)] = &[
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    #[test]
    fn snafu_to_decimal() {
        for &(dec, snaf) in EXAMPLES {
            let parsed: SnafuNum = snaf.parse().unwrap();
            assert_eq!(parsed.0, dec);
        }
    }

    #[test]
    fn decimal_to_snafu() {
        for &(dec, snaf) in EXAMPLES {
            let formatted = SnafuNum(dec).to_string();
            assert_eq!(snaf, &formatted);
        }
    }
}

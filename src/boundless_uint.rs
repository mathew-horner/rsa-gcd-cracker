use std::{ops::Div, str::FromStr};

use anyhow::anyhow;

/// An unsigned integer that is not bounded by the CPU word size.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BoundlessUint(Vec<u8>);

/// Euclid's Greatest Common Divisor algorithm.
pub fn euclid(n1: &BoundlessUint, n2: &BoundlessUint) -> BoundlessUint {
    todo!();
}

impl FromStr for BoundlessUint {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits = s
            .chars()
            .map(|c| {
                c.to_digit(10)
                    .map(|d| d as u8)
                    .ok_or_else(|| anyhow!("non digit character encountered in input: {c}"))
            })
            .collect::<Result<_, _>>()?;

        Ok(Self(digits))
    }
}

// NOTE: An implementation only on u32 seems unnecessarily restrictive, but as of now, we only ever
// call this with literal values anyways.
impl From<u32> for BoundlessUint {
    fn from(mut value: u32) -> Self {
        let mut digits = Vec::new();
        while value > 0 {
            let digit = (value % 10) as u8;
            value /= 10;
            digits.push(digit);
        }
        digits.reverse();
        Self(digits)
    }
}

impl Div for &BoundlessUint {
    type Output = BoundlessUint;

    fn div(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn parse_valid_big_number() {
        assert_eq!(
            "53267".parse::<BoundlessUint>().unwrap(),
            BoundlessUint(vec![5, 3, 2, 6, 7])
        );
    }

    #[test]
    fn parse_invalid_big_number() {
        assert_eq!(
            "12t65".parse::<BoundlessUint>().unwrap_err().to_string(),
            "non digit character encountered in input: t",
        )
    }

    #[test]
    fn convert_u32_to_big_number() {
        assert_eq!(BoundlessUint::from(4567), BoundlessUint(vec![4, 5, 6, 7]));
    }

    #[test]
    fn big_number_division() {
        assert_eq!(
            &BoundlessUint::from(1224) / &BoundlessUint::from(18),
            BoundlessUint::from(68),
        );
    }
}

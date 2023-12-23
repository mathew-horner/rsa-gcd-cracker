use openssl::bn::BigNum;

/// Euclid's Greatest Common Divisor algorithm.
pub fn euclid(mut n1: BigNum, mut n2: BigNum) -> BigNum {
    while n1 != n2 {
        if n1 > n2 {
            let d = n1.as_ref() - n2.as_ref();
            n1 = d;
        } else {
            let d = n2.as_ref() - n1.as_ref();
            n2 = d;
        }
    }
    n1
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn euclids_gcd() {
        let cases = [
            (
                (
                    BigNum::from_u32(18).unwrap(),
                    BigNum::from_u32(24_u32).unwrap(),
                ),
                BigNum::from_u32(6_u32).unwrap(),
            ),
            (
                (
                    BigNum::from_u32(88_u32).unwrap(),
                    BigNum::from_u32(99_u32).unwrap(),
                ),
                BigNum::from_u32(11_u32).unwrap(),
            ),
            (
                (
                    BigNum::from_dec_str("12345678901234567890").unwrap(),
                    BigNum::from_dec_str("98765432109876543210").unwrap(),
                ),
                BigNum::from_dec_str("900000000090").unwrap(),
            ),
        ];

        for ((n1, n2), expected_gcd) in cases {
            assert_eq!(euclid(n1, n2), expected_gcd);
        }
    }
}

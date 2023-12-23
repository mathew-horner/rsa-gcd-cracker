use std::{fs, path::Path};

use openssl::bn::{BigNum, BigNumContext};
use openssl::pkey::{Private, Public};
use openssl::rsa::{Padding, Rsa};

use crate::math::euclid;

pub struct Challenge {
    pub number: usize,
    pub public_key: Rsa<Public>,
    pub encrypted_message: Vec<u8>,
}

impl Challenge {
    pub fn read(number: usize) -> Self {
        let pem = fs::read(Path::new(&format!("challenge/{number}.pem"))).unwrap();
        let encrypted_message = fs::read(Path::new(&format!("challenge/{number}.bin"))).unwrap();
        Self {
            number,
            public_key: Rsa::public_key_from_pem(&pem).unwrap(),
            encrypted_message,
        }
    }

    /// Attempt to crack two RSA public keys using a Common Factor Attack.
    ///
    /// This method relies upon using GCD to discover shared factors between two keys (p or q).
    /// If a shared factor is found, the other two are easily found by dividing each respective
    /// public key by the shared factor.
    ///
    /// Though it is not common, some historical bugs in random number generators can lead to
    /// factors being re-used between keys.
    pub fn attempt(&self, other: &Challenge) -> Option<(Solution, Solution)> {
        let gcd = euclid(
            self.public_key.n().to_owned().unwrap(),
            other.public_key.n().to_owned().unwrap(),
        );

        // If GCD is 1, there is no shared prime between the keys and thus a crack is unfeasible.
        if gcd == BigNum::from_u32(1).unwrap() {
            return None;
        }

        // If GCD is *not* 1, it is the shared prime between the keys and can be used to determine the others.
        let a = self.public_key.n() / gcd.as_ref();
        let c = other.public_key.n() / gcd.as_ref();

        Some((
            Solution::solve(
                self.number,
                build_private_key(
                    self.public_key.n().to_owned().unwrap(),
                    self.public_key.e().to_owned().unwrap(),
                    gcd.as_ref().to_owned().unwrap(),
                    a,
                ),
                &self.encrypted_message,
            ),
            Solution::solve(
                other.number,
                build_private_key(
                    other.public_key.n().to_owned().unwrap(),
                    other.public_key.e().to_owned().unwrap(),
                    gcd,
                    c,
                ),
                &other.encrypted_message,
            ),
        ))
    }
}

pub struct Solution {
    pub challenge: usize,
    pub private_key: Rsa<Private>,
    pub decrypted_message: String,
}

impl Solution {
    fn solve(challenge_number: usize, private_key: Rsa<Private>, encrypted_message: &[u8]) -> Self {
        let mut buf = [0; 128];
        let size = private_key
            .private_decrypt(encrypted_message, &mut buf, Padding::PKCS1)
            .unwrap();
        Self {
            challenge: challenge_number,
            private_key,
            decrypted_message: String::from_utf8(Vec::from(&buf[..size])).unwrap(),
        }
    }
}

fn build_private_key(n: BigNum, e: BigNum, p: BigNum, q: BigNum) -> Rsa<Private> {
    let one = BigNum::from_u32(1).unwrap();
    let p1 = &p - &one;
    let q1 = &q - &one;
    let phi = &p1 * &q1;
    let mut d = BigNum::new().unwrap();
    d.mod_inverse(&e, &phi, &mut BigNumContext::new().unwrap())
        .unwrap();

    let dmp1 = &d % &p1;
    let dmq1 = &d % &q1;

    let mut iqmp = BigNum::new().unwrap();
    iqmp.mod_inverse(&q, &p, &mut BigNumContext::new().unwrap())
        .unwrap();

    Rsa::from_private_components(n, e, d, p, q, dmp1, dmq1, iqmp).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn produces_solution_for_known_key() {
        let message = "Hello World!";
        let private_key = Rsa::generate(1024).unwrap();

        let mut buf = [0; 128];
        private_key
            .public_encrypt(message.as_bytes(), &mut buf, Padding::PKCS1)
            .unwrap();

        let solution = Solution::solve(
            0,
            build_private_key(
                private_key.n().to_owned().unwrap(),
                private_key.e().to_owned().unwrap(),
                private_key.p().unwrap().to_owned().unwrap(),
                private_key.q().unwrap().to_owned().unwrap(),
            ),
            &buf,
        );

        assert_eq!(solution.decrypted_message, message);
    }
}

use crate::boundless_uint::{euclid, BoundlessUint};

/// Contains a public / private RSA key pair.
#[derive(Debug, Eq, PartialEq)]
pub struct KeyPair {
    pub private: PrivateKey,
    pub public: BoundlessUint,
}

/// Attempt to crack two RSA public keys using a Common Factor Attack.
///
/// This method relies upon using GCD to discover shared factors between two keys (p or q).
/// If a shared factor is found, the other two are easily found by dividing each respective
/// public key by the shared factor.
///
/// Though it is not common, some historical bugs in random number generators can lead to
/// factors being re-used between keys.
pub fn attempt_crack(
    public_key1: BoundlessUint,
    public_key2: BoundlessUint,
) -> Option<(KeyPair, KeyPair)> {
    let gcd = euclid(&public_key1, &public_key2);

    // If GCD is 1, there is no shared prime between the keys and thus a crack is unfeasible.
    if gcd == BoundlessUint::from(1) {
        return None;
    }

    // If GCD is *not* 1, it is the shared prime between the keys and can be used to determine the others.
    let shared = gcd;
    let left = &public_key1 / &shared;
    let right = &public_key2 / &shared;

    Some((
        KeyPair {
            private: PrivateKey(shared.clone(), left),
            public: public_key1,
        },
        KeyPair {
            private: PrivateKey(shared, right),
            public: public_key2,
        },
    ))
}

/// Contains the p and q values that make up an RSA private key, though distinguishing between
/// the two is of no consequence to this program.
#[derive(Debug)]
pub struct PrivateKey(BoundlessUint, BoundlessUint);

impl PartialEq for PrivateKey {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 || self.0 == other.1 && self.1 == other.0
    }
}

impl Eq for PrivateKey {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn private_key_comparison() {
        assert_eq!(
            PrivateKey(BoundlessUint::from(13), BoundlessUint::from(53)),
            PrivateKey(BoundlessUint::from(13), BoundlessUint::from(53))
        );
    }

    #[test]
    fn private_key_comparison_flipped() {
        assert_eq!(
            PrivateKey(BoundlessUint::from(13), BoundlessUint::from(53)),
            PrivateKey(BoundlessUint::from(53), BoundlessUint::from(13))
        );
    }

    #[test]
    fn crack_keys_with_shared_prime() {
        // 13 is the shared prime.
        let public_key1 = BoundlessUint::from(13 * 53);
        let public_key2 = BoundlessUint::from(13 * 97);

        let (pair1, pair2) = attempt_crack(public_key1.clone(), public_key2.clone()).unwrap();

        assert_eq!(
            pair1,
            KeyPair {
                private: PrivateKey(BoundlessUint::from(13), BoundlessUint::from(53)),
                public: public_key1
            }
        );
        assert_eq!(
            pair2,
            KeyPair {
                private: PrivateKey(BoundlessUint::from(13), BoundlessUint::from(97)),
                public: public_key2
            }
        );
    }

    #[test]
    fn crack_keys_without_shared_prime() {
        assert_eq!(
            attempt_crack(BoundlessUint::from(13 * 29), BoundlessUint::from(53 * 97)),
            None
        );
    }
}

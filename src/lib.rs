//! Defines data structures representing Diffie-Hellman Groups stated in [RFC3526](https://datatracker.ietf.org/doc/rfc3526/)

pub mod element;
pub mod group;

#[cfg(test)]
mod test {
    #![allow(non_snake_case)]

    use std::str::FromStr;

    use num_bigint::BigUint;

    use crate::{
        element::Element,
        group::{MODPGroup5, PrimeGroup},
    };

    fn test_key_exchange<G: PrimeGroup>() {
        // A = g^a mod p
        let a = BigUint::from_str("2").unwrap();
        let A = Element::<G>::from_biguint(a.clone());
        // B = g^b mod p
        let b = BigUint::from_str("3").unwrap();
        let B = Element::<G>::from_biguint(b.clone());

        // shared secret = B^a = A^b mod p
        let s = B.pow(&a);
        let z = A.pow(&b);

        assert_eq!(s, z)
    }

    #[test]

    fn test_key_exchange_group_1() {
        test_key_exchange::<MODPGroup5>();
    }

    #[test]
    fn test_key_exchange_group_14() {
        test_key_exchange::<crate::group::MODPGroup14>();
    }

    #[test]
    fn test_key_exchange_group_15() {
        test_key_exchange::<crate::group::MODPGroup15>();
    }

    #[test]
    fn test_key_exchange_group_16() {
        test_key_exchange::<crate::group::MODPGroup16>();
    }

    #[test]
    fn test_key_exchange_group_17() {
        test_key_exchange::<crate::group::MODPGroup17>();
    }

    #[test]
    fn test_key_exchange_group_18() {
        test_key_exchange::<crate::group::MODPGroup18>();
    }
}

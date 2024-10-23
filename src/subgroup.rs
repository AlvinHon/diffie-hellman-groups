use num_bigint::{BigUint, RandomBits};
use rand::Rng;

use crate::MODPGroup;

/// Subgroup of a MODP group. It shares the same prime modulus and order as the parent group,
/// but has a different generator `g` such that g^q mod p = 1.
#[derive(Clone, Debug)]
pub struct SubGroup {
    /// Prime modulus.
    pub p: BigUint,
    /// Order of the subgroup, a prime number.
    pub q: BigUint,
    /// Generator of the subgroup, g^q mod p = 1.
    pub g: BigUint,
}

impl SubGroup {
    /// Create a new subgroup with a generator of `num_bits` bits.
    ///
    /// # Panics
    /// Panics if `num_bits` is less than 2 or greater than the number of bits in the order of the group.
    ///
    /// # Example
    ///
    /// ```rust
    /// use diffie_hellman_groups::{SubGroup, group::{MODPGroup, MODPGroup5}};
    ///
    /// let subg = SubGroup::new::<MODPGroup5>(128);
    /// println!("{:?}", subg);
    /// assert!(subg.g != MODPGroup5::generator());
    /// ```
    pub fn new<G: MODPGroup>(num_bits: usize) -> Self {
        let p = G::prime_modulus();
        let q = G::order();
        assert!(num_bits >= 2 && num_bits <= q.bits());
        let g;

        let rng = &mut rand::thread_rng();
        loop {
            let a = rng.sample::<BigUint, _>(RandomBits::new(num_bits));
            if a != G::generator() {
                let res = a.modpow(&q, &p);
                if res == BigUint::from(1u64) {
                    g = a;
                    break;
                }
            }
        }

        Self { p, q, g }
    }
}

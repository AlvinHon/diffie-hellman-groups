use num_bigint::{BigUint, RandomBits};
use num_prime::{nt_funcs, Primality};
use rand::Rng;

use crate::MODPGroup;

/// PrimeGroup represents a group of a prime order `q` of a group with a prime modulus `p`,
/// and a generator `g` such that g^q mod p = 1.
#[derive(Clone, Debug)]
pub struct PrimeGroup {
    /// Prime modulus.
    pub p: BigUint,
    /// Order of the group, a prime number.
    pub q: BigUint,
    /// Generator of the group, g^q mod p = 1.
    pub g: BigUint,
}

impl PrimeGroup {
    /// Create a new group from `MODPGroup` with a different generator of `num_bits` bits.
    ///
    /// # Panics
    /// Panics if `num_bits` is less than 2 or greater than the number of bits in the prime modulus `p`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use diffie_hellman_groups::{PrimeGroup, group::{MODPGroup, MODPGroup5}};
    ///
    /// let pg = PrimeGroup::new::<MODPGroup5>(128);
    /// println!("{:?}", pg);
    /// assert!(pg.g != MODPGroup5::generator());
    /// ```
    pub fn new<G: MODPGroup>(num_bits: usize) -> Self {
        let p = G::prime_modulus();
        let q = G::sophie_garmain_prime();
        assert!(num_bits >= 2 && num_bits <= p.bits() as usize);
        let g;

        let rng = &mut rand::thread_rng();
        loop {
            let a = rng.sample::<BigUint, _>(RandomBits::new(num_bits as u64));
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

    /// Create a new prime group with a generator of `generator_num_bits` bits and order of `order_num_bits` bits.
    /// The prime modulus `q` is set such that p = 2q + 1 . The generator `g` is generated randomly such that
    /// g^q mod p = 1.
    ///
    /// # Panics
    /// Panics if
    /// - `generator_num_bits` is less than 2 or greater than the number of bits in the prime modulus `p`.
    /// - `p` is not a safe prime.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_bigint::BigUint;
    /// use diffie_hellman_groups::PrimeGroup;
    ///
    /// let p = BigUint::from(1623299u64);
    /// let pg = PrimeGroup::new_with(p, 15);
    /// println!("{:?}", pg);
    /// ```
    pub fn new_with(p: BigUint, generator_num_bits: usize) -> Self {
        assert!(generator_num_bits >= 2);
        assert!(generator_num_bits <= p.bits() as usize);
        assert!(nt_funcs::is_safe_prime(&p) == Primality::Yes);

        // q is a sophie germain prime
        let q = (&p - BigUint::from(1u64)) / BigUint::from(2u64);
        let g;
        let rng = &mut rand::thread_rng();
        loop {
            let a = rng.sample::<BigUint, _>(RandomBits::new(generator_num_bits as u64));
            let res = a.modpow(&q, &p);
            if res == BigUint::from(1u64) {
                g = a;
                break;
            }
        }
        Self { p, q, g }
    }
}

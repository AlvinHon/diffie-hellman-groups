use std::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use num_bigint::BigUint;
use serde::{Deserialize, Serialize};

use crate::group::MODPGroup;

/// An element of a MODP group, implemented as a wrapper around a BigUint.
///
/// This struct implements the Add, Sub, and Mul traits, allowing for arithmetic operations on elements of a MODP group.
///
/// # Example
///
/// ```rust
/// use num_bigint::BigUint;
/// use diffie_hellman_groups::{Element, group::MODPGroup5};
///
///
/// let a = Element::<MODPGroup5>::from_biguint(BigUint::from(2u32)); // = g^2 mod p
/// let b = Element::<MODPGroup5>::from_biguint(BigUint::from(3u32)); // = g^3 mod p
///
/// let lhs = a * b;
/// let rhs = Element::<MODPGroup5>::from_biguint(BigUint::from(5u32)); // = g^5 mod p
/// assert_eq!(lhs, rhs);
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Element<G: MODPGroup> {
    pub value: BigUint,
    phantom: std::marker::PhantomData<G>,
}

impl<G: MODPGroup> Element<G> {
    /// Create an Element from a BigUint. Returns the value = g^value mod p, where g and p
    /// are the generator and modulus of the group.
    pub fn from_biguint(value: BigUint) -> Self {
        Element {
            value: G::element(&value),
            phantom: std::marker::PhantomData,
        }
    }

    /// Raise the element to the power of the exponent. Returns the value = self.value^exponent mod p,
    /// where p is the modulus of the group.
    ///
    /// # Example
    ///
    /// ```rust
    /// use num_bigint::BigUint;
    /// use diffie_hellman_groups::{Element, group::MODPGroup5};
    ///
    /// let a = Element::<MODPGroup5>::from_biguint(BigUint::from(2u32)); // = g^2 mod p
    /// let b = BigUint::from(3u32);
    ///
    /// let lhs = a.pow(&b);
    /// let rhs = Element::<MODPGroup5>::from_biguint(BigUint::from(6u32)); // = g^6 mod p
    /// assert_eq!(lhs, rhs);
    /// ```
    pub fn pow(&self, exponent: &BigUint) -> Self {
        Element {
            value: G::pow(&self.value, exponent),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> PartialEq for Element<G> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<G: MODPGroup> Eq for Element<G> {}

impl<G: MODPGroup> AsRef<BigUint> for Element<G> {
    fn as_ref(&self) -> &BigUint {
        &self.value
    }
}

impl<G: MODPGroup> FromStr for Element<G> {
    type Err = num_bigint::ParseBigIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Element {
            value: G::element(&BigUint::from_str(s)?),
            phantom: std::marker::PhantomData,
        })
    }
}

// Implementing the Add, Sub, and Mul traits for the Element struct

impl<G: MODPGroup> Add for Element<G> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Element {
            value: G::add(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Add for &Element<G> {
    type Output = Element<G>;

    fn add(self, rhs: Self) -> Self::Output {
        Element {
            value: G::add(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Add<&Element<G>> for Element<G> {
    type Output = Element<G>;

    fn add(self, rhs: &Element<G>) -> Self::Output {
        Element {
            value: G::add(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Add<Element<G>> for &Element<G> {
    type Output = Element<G>;

    fn add(self, rhs: Element<G>) -> Self::Output {
        Element {
            value: G::add(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Sub for Element<G> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Element {
            value: G::sub(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Sub for &Element<G> {
    type Output = Element<G>;

    fn sub(self, rhs: Self) -> Self::Output {
        Element {
            value: G::sub(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Sub<Element<G>> for &Element<G> {
    type Output = Element<G>;

    fn sub(self, rhs: Element<G>) -> Self::Output {
        Element {
            value: G::sub(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Sub<&Element<G>> for Element<G> {
    type Output = Element<G>;

    fn sub(self, rhs: &Element<G>) -> Self::Output {
        Element {
            value: G::sub(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Mul for Element<G> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Element {
            value: G::mul(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Mul for &Element<G> {
    type Output = Element<G>;

    fn mul(self, rhs: Self) -> Self::Output {
        Element {
            value: G::mul(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Mul<&Element<G>> for Element<G> {
    type Output = Element<G>;

    fn mul(self, rhs: &Element<G>) -> Self::Output {
        Element {
            value: G::mul(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: MODPGroup> Mul<Element<G>> for &Element<G> {
    type Output = Element<G>;

    fn mul(self, rhs: Element<G>) -> Self::Output {
        Element {
            value: G::mul(&self.value, &rhs.value),
            phantom: std::marker::PhantomData,
        }
    }
}

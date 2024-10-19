use std::{
    ops::{Add, Mul, Sub},
    str::FromStr,
};

use num_bigint::BigUint;

use crate::group::MODPGroup;

#[derive(Debug, Clone)]
pub struct Element<G: MODPGroup> {
    pub value: BigUint,
    phantom: std::marker::PhantomData<G>,
}

impl<G: MODPGroup> Element<G> {
    pub fn from_biguint(value: BigUint) -> Self {
        Element {
            value: G::element(&value),
            phantom: std::marker::PhantomData,
        }
    }
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

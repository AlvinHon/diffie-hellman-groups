use std::str::FromStr;

use num_bigint::BigUint;

use crate::group::PrimeGroup;

#[derive(Debug, Clone)]
pub struct Element<G: PrimeGroup> {
    pub value: BigUint,
    phantom: std::marker::PhantomData<G>,
}

impl<G: PrimeGroup> Element<G> {
    pub fn from_str(value: &str) -> Self {
        Element {
            value: G::element(&BigUint::from_str(value).unwrap()),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn from_biguint(value: BigUint) -> Self {
        Element {
            value: G::element(&value),
            phantom: std::marker::PhantomData,
        }
    }
    pub fn pow(&self, exponent: &BigUint) -> Self {
        Element {
            value: G::op(&self.value, &exponent),
            phantom: std::marker::PhantomData,
        }
    }
}

impl<G: PrimeGroup> PartialEq for Element<G> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl<G: PrimeGroup> Eq for Element<G> {}

impl<G: PrimeGroup> AsRef<BigUint> for Element<G> {
    fn as_ref(&self) -> &BigUint {
        &self.value
    }
}

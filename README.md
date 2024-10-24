# Diffie-Hellman Groups

This library provides data structures representing Diffie-Hellman Groups stated in [RFC3526](https://datatracker.ietf.org/doc/rfc3526/). The data structures are useful in implementing cryptographic protocols such as Internet Key Exchange (IKE).

## Prime Constants

It provides constants of the primes in typed `BigUint` given by dependency [num-bigint](https://crates.io/crates/num-bigint). For example, you can get the value by calling a method in provided structs that implements the trait `MODPGroup`.

```rust
MODPGroup5::prime_modulus();
```

## Arithmetic Elements

The struct `Element` represents an element in the MODP Group, which implements traits in `std::ops` for arithmetic operations.

It is an example to illustrate the secret sharing in Diffie-Hellman Key Exchange.

```rust
// A = g^a mod p
let a = BigUint::from_str("2").unwrap();
let A = Element::<MODPGroup5>::from_biguint(a.clone());
// B = g^b mod p
let b = BigUint::from_str("3").unwrap();
let B = Element::<MODPGroup5>::from_biguint(b.clone());

// shared secret = B^a = A^b mod p
let s = B.pow(&a);
let z = A.pow(&b);

assert_eq!(s, z)
```

## Prime Group

The library can provide a so-called `Prime Group` when enabling the feature `primegroup`. Tit is the struct `PrimeGroup` that represents a MODP Group such that: 

```text
p is prime modulus, which is a safe prime.
q is also a prime s.t. p = 2q+1 mod p
g is a generator s.t. g^q mod p = 1
```

See [Wiki](https://en.wikipedia.org/wiki/Safe_and_Sophie_Germain_primes) for details about `p` and `q`.

The provided MODP Groups are having safe primes as modulus. You can create a group from them, for example,

```rust
PrimeGroup::new::<MODPGroup5>(128); // a 128-bit generator
``` 
// imports A collection of numeric types and traits for Rust.
// check cargo.toml [dependencies]
extern crate num;
use num::bigint::BigInt;
use num::Integer;
use num::One;
use num::Zero;

/// The Rust Program calculates the Modular multiplicative inverse of a number
///  using the Euclidean Algorithm.
/// The Euclidean Algorithm is a set of instructions for finding the greatest common divisor
/// of any two  integers.

/// Modular multiplicative inverse function for big (negative) numbers

/// This function is an implementation of the [extended Euclidean
/// algorithm](https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm).

// Creates and initializes a BigInt.
fn modinv(n: &BigInt, p: &BigInt) -> BigInt {
    if p.is_one() {
        return BigInt::one();
    }
    /// the definition of a, m, x, and inv.
    let (mut a, mut m, mut x, mut inv) = (n.clone(), p.clone(), BigInt::zero(), BigInt::one());

    while a < BigInt::zero() {
        a += p
    }

    while a > BigInt::one() {
        let (div, rem) = a.div_rem(&m);
        inv -= div * &x;
        a = rem;
        std::mem::swap(&mut a, &mut m);
        std::mem::swap(&mut x, &mut inv);
    }

    if inv < BigInt::zero() {
        inv += p
    }
    inv
}

fn main() {
    ///  A BigInt value, also sometimes just called a BigInt, is a bigint primitive,
    /// created by appending n to the end of an integer literal.
    let n = BigInt::parse_bytes(
        b"243772585612020160733370897338805215918303827399330592839196552441720391139",
        10,
    )
    .unwrap();
    let p = BigInt::parse_bytes(
        b"115792089237316195423570985008687907853269984665640564039457584007908834671663",
        10,
    )
    .unwrap();

    println!("modinv({0}, {1}) = {2}", n, p, modinv(&n, &p)); // Print Modular multiplicative inverse
}

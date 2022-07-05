// #![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the BLS12_381_new curve.
//! The name denotes that it is a Barreto--Lynn--Scott curve of embedding degree
//! 12, defined over a 381-bit (prime) field.
//! This curve was intended to replace the BLS12_377 curve to provide a better
//! efficiency in terms of hash circuits and scalar multiplications.
//!
//! Curve information:
//! * Base field: q = 2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154753
//! * Scalar field: r =
//!   40134810535214015562426085132763902269106966834552711290100314126475667177473
//! * valuation(q - 1, 2) = 40
//! * valuation(r - 1, 2) = 41
//! * G1 curve equation: y^2 = x^3 + 1
//! * G2 curve equation: y^2 = x^3 + Fq2(4, 4)

#[cfg(feature = "curve")]
mod curves;
mod fields;

#[cfg(feature = "curve")]
pub use curves::*;
pub use fields::*;

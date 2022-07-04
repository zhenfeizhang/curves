#![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements a twisted Edwards curve whose base field is the
//! scalar field of the curve BLS12-381-new.  This allows defining cryptographic
//! primitives that use elliptic curves over the scalar field of the latter
//! curve.
//!
//! Curve information:
//! * Base field: q =
//! 40134810535214015562426085132763902269106966834552711290100314126475667177473
//! * Scalar field: r =
//! 1114855848200389321178502364798997285263996944004370985025370127112229300857
//! * Valuation(q - 1, 2) = 41
//! * Valuation(r - 1, 2) = 3
//! * Curve equation: ax^2 + y^2 =1 + dx^2y^2, where
//!    * a = -1
//!    * d = 14981574235622264949562026455995287857343562316607173614446448398092105601444

#[cfg(feature = "r1cs")]
pub mod constraints;
mod curves;
mod fields;

pub use curves::*;
pub use fields::*;

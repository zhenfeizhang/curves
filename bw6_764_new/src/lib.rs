// #![cfg_attr(not(feature = "std"), no_std)]
#![deny(
    warnings,
    unused,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

//! This library implements the BW6_764 curve.
//! The name denotes that it is a curve generated using the Brezing--Weng
//! method, and that its embedding degree is 6.
//! The main feature of this curve is that the scalar field equals the base
//! field of the BLS12_381_new curve.
//!
//! Curve information:
//! * Base field: q = 50282768576993852407494634308294841376983574838385858405447848583376065247544506093833501024756151257915195555055355085817082514603850271637792434072458297604453662538937558570183541064674323908560052492735693432884208755873415169
//! * Scalar field: r = 2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154753
//! * valuation(q - 1, 2) = 42
//! * valuation(r - 1, 2) = 40
//!
//! G1 curve equation: y^2 = x^3 + ax + b, where
//! * a = 0,
//! * b = 3,
//!
//! G2 curve equation: y^2 = x^3 + Ax + B
//! * A = 0
//! * B = 33

mod curves;
mod fields;

pub use curves::*;
pub use fields::*;

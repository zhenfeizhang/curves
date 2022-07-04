use crate::*;
use ark_ec::{
    bls12,
    models::{ModelParameters, SWModelParameters},
};
use ark_ff::{field_new, Zero};

pub type G1Affine = bls12::G1Affine<crate::Parameters>;
pub type G1Projective = bls12::G1Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 4
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "1");

    /// COFACTOR = (x - 1)^2 / 3  = 66778914282889904464656736638700879872
    const COFACTOR: &'static [u64] = &[0, 0x323d26ac55830000];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// = 17013441151896256258591749887919689498413710868588296929277
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "17013441151896256258591749887919689498413710868588296929277");

    /// AFFINE_GENERATOR_COEFFS = (G1_GENERATOR_X, G1_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G1_GENERATOR_X, G1_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

/// G1_GENERATOR_X =
/// 1825189513136555147728161400146776469617962153064438236682800310735583776602910731904434256739957045018687259224622 
#[rustfmt::skip]
pub const G1_GENERATOR_X: Fq = field_new!(Fq, "1825189513136555147728161400146776469617962153064438236682800310735583776602910731904434256739957045018687259224622");

/// G1_GENERATOR_Y =
/// 1178115723215551622681068137844479075836183778111297851090273558296913094605027167230300014768906477814621978580327
#[rustfmt::skip]
pub const G1_GENERATOR_Y: Fq = field_new!(Fq, "1178115723215551622681068137844479075836183778111297851090273558296913094605027167230300014768906477814621978580327");

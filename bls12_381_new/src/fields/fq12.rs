use crate::*;
use ark_ff::{field_new, fields::*};

pub type Fq12 = Fp12<Fq12Parameters>;

#[derive(Clone, Copy)]
pub struct Fq12Parameters;

impl Fp12Parameters for Fq12Parameters {
    type Fp6Params = Fq6Parameters;

    const NONRESIDUE: Fq6 = field_new!(
        Fq6,
        field_new!(Fq2, field_new!(Fq, "2"), field_new!(Fq, "0"),),
        field_new!(Fq2, field_new!(Fq, "3"), field_new!(Fq, "0"),),
        field_new!(Fq2, field_new!(Fq, "2"), field_new!(Fq, "0"),),
    );

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
        field_new!(Fq2, field_new!(Fq, "0"), field_new!(Fq, "0"),),
    ];
}

use crate::*;
use ark_ff::{field_new, fields::*};

pub type Fq2 = Fp2<Fq2Parameters>;

pub struct Fq2Parameters;

impl Fp2Parameters for Fq2Parameters {
    type Fp = Fq;

    /// NONRESIDUE = -5
    // it is a Fq non square element defining Fq2/Fq
    #[rustfmt::skip]
    const NONRESIDUE: Fq = field_new!(Fq, "-5");

    /// QUADRATIC_NONRESIDUE = 3+2*u
    // it is actually a non-cube... the name is not well chosen. and it defines Fq6 / Fq2
    // I don't think this value is useful here...
    #[rustfmt::skip]
    const QUADRATIC_NONRESIDUE: (Fq, Fq) = (
        field_new!(Fq, "3"),
        field_new!(Fq, "2"),
    );

    /// Coefficients for the Frobenius automorphism.
    // these are NONRESIDUE**((q^j-1)/2)
    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP2_C1: &'static [Fq] = &[
        // Fq(-5)**(((q^0) - 1) / 2)
        field_new!(Fq, "1"),
        // Fq(-5)**(((q^1) - 1) / 2)
        field_new!(Fq, "-1"),
    ];

    #[inline(always)]
    fn mul_fp_by_nonresidue(fe: &Self::Fp) -> Self::Fp {
        let original = fe;
        let mut fe = -fe.double();
        fe.double_in_place();
        fe - original
    }
}

pub const FQ2_ZERO: Fq2 = field_new!(Fq2, FQ_ZERO, FQ_ZERO);
pub const FQ2_ONE: Fq2 = field_new!(Fq2, FQ_ONE, FQ_ZERO);

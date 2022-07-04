use crate::{Fq, Fq3, Fq3Parameters, FQ_ONE};
use ark_ff::{
    field_new,
    fields::fp6_2over3::{Fp6, Fp6Parameters},
};

pub type Fq6 = Fp6<Fq6Parameters>;

pub struct Fq6Parameters;

impl Fp6Parameters for Fq6Parameters {
    type Fp3Params = Fq3Parameters;

    /// NONRESIDUE = (1, 3, 2)
    #[rustfmt::skip]
    const NONRESIDUE: Fq3 = field_new!(
        Fq3,
        FQ_ONE,
        field_new!(Fq, "3"),
        field_new!(Fq, "2"),
    );

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C1: &'static [Fq] = &[
        field_new!(Fq, "0"),
        field_new!(Fq, "0"),
        field_new!(Fq, "0"),
        field_new!(Fq, "0"),
        field_new!(Fq, "0"),
        field_new!(Fq, "0"),
    ];
}

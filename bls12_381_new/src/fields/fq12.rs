use crate::*;
use ark_ff::{field_new, fields::*};

pub type Fq12 = Fp12<Fq12Parameters>;

#[derive(Clone, Copy)]
pub struct Fq12Parameters;

impl Fp12Parameters for Fq12Parameters {
    type Fp6Params = Fq6Parameters;

    // not useful???
    const NONRESIDUE: Fq6 = field_new!(
        Fq6,
        field_new!(Fq2, field_new!(Fq, "2"), field_new!(Fq, "0"),),
        field_new!(Fq2, field_new!(Fq, "3"), field_new!(Fq, "0"),),
        field_new!(Fq2, field_new!(Fq, "2"), field_new!(Fq, "0"),),
    );

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
        // Fp2::NONRESIDUE^(((q^0) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "1"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^1) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "2449665355933841550053749794782130116254807162379321632558450792789517359698962924609217926589703681677699117246500"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^2) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154752"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^3) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "230493716557241884797954946469706661009015338835221120954706674153932244368975053017203575832847097136810864908253"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^4) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "1"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^5) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "2449665355933841550053749794782130116254807162379321632558450792789517359698962924609217926589703681677699117246500"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^6) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154752"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^7) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "230493716557241884797954946469706661009015338835221120954706674153932244368975053017203575832847097136810864908253"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^8) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "1"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^9) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "2449665355933841550053749794782130116254807162379321632558450792789517359698962924609217926589703681677699117246500"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^10) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154752"),
            field_new!(Fq, "0"),
        ),
        // Fp2::NONRESIDUE^(((q^11) - 1) / 6)
        field_new!(Fq2,
            field_new!(Fq, "230493716557241884797954946469706661009015338835221120954706674153932244368975053017203575832847097136810864908253"),
            field_new!(Fq, "0"),
        ),
    ];
}

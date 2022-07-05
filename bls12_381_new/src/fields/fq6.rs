use crate::*;
use ark_ff::{field_new, fields::*};

pub type Fq6 = Fp6<Fq6Parameters>;

#[derive(Clone, Copy)]
pub struct Fq6Parameters;

impl Fp6Parameters for Fq6Parameters {
    type Fp2Params = Fq2Parameters;

    /// NONRESIDUE = U
    #[rustfmt::skip]
    const NONRESIDUE: Fq2 = field_new!(Fq2,
		FQ_ZERO, FQ_ONE
    );

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C1: &'static [Fq2] = &[
		// Fp2(u+1)^(((q^0) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((q^1) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154752"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((q^2) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((q^3) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154752"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((q^4) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((q^5) - 1) / 3)
		field_new!(Fq2,
			field_new!(Fq, "2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154752"),
			field_new!(Fq, "0"),
		),
    ];

    #[rustfmt::skip]
    const FROBENIUS_COEFF_FP6_C2: &'static [Fq2] = &[
		// Fp2(u+1)^(((2q^0) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((2q^1) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((2q^2) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((2q^3) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((2q^4) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
		// Fp2(u+1)^(((2q^5) - 2) / 3)
		field_new!(Fq2,
			field_new!(Fq, "1"),
			field_new!(Fq, "0"),
		),
    ];

    /// Multiply this element by the quadratic nonresidue u.
    /// Make this generic.
    fn mul_fp2_by_nonresidue(fe: &Fq2) -> Fq2 {
        // (x+uy) * u = xu + u²y = -5y + u*x
        let mut copy = *fe;
        let t0 = copy.c0;
        copy.c0 = -fe.c1.double().double() - fe.c1;
        copy.c1 = t0;
        copy
    }
}

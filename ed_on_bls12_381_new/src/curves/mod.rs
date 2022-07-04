use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    models::{ModelParameters, MontgomeryModelParameters, TEModelParameters},
    twisted_edwards_extended::{GroupAffine, GroupProjective},
};
use ark_ff::{field_new, Field};

#[cfg(test)]
mod tests;

pub type EdwardsAffine = GroupAffine<EdwardsParameters>;
pub type EdwardsProjective = GroupProjective<EdwardsParameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct EdwardsParameters;

impl ModelParameters for EdwardsParameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl TEModelParameters for EdwardsParameters {
    /// COEFF_A = -1
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "5");

    /// COEFF_D = 5361749892316706377042037985551365251496122086069554507968386262490806347726
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "5361749892316706377042037985551365251496122086069554507968386262490806347726");

    /// COFACTOR = 36
    const COFACTOR: &'static [u64] = &[36];

    /// COFACTOR_INV =
    /// 340650398061230070360097944799693614941776844001335578757751983284292286373
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "340650398061230070360097944799693614941776844001335578757751983284292286373");

    /// Generated randomly
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) = (GENERATOR_X, GENERATOR_Y);

    type MontgomeryModelParameters = EdwardsParameters;

    /// Multiplication by `a` is a multiplication by 5.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        let t = (*elem).double().double();
        t + *elem
    }
}

impl MontgomeryModelParameters for EdwardsParameters {
    /// COEFF_A = 22968850243239004857190673804004204626556472040028799126863518348148555936956
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "22968850243239004857190673804004204626556472040028799126863518348148555936956");
    /// COEFF_B = 31383373346985961179567629233592058548641804656514037501899063450248652281756
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "31383373346985961179567629233592058548641804656514037501899063450248652281756");

    type TEModelParameters = EdwardsParameters;
}

/// GENERATOR_X =
/// 15120448893185571816904166627167789370828130373238443616975311570904497857862,
#[rustfmt::skip]
const GENERATOR_X: Fq = field_new!(Fq, "15120448893185571816904166627167789370828130373238443616975311570904497857862");

/// GENERATOR_Y =
/// 36855568516631485357775733204777558943452690195180170433918781310838574341943
#[rustfmt::skip]
const GENERATOR_Y: Fq = field_new!(Fq, "36855568516631485357775733204777558943452690195180170433918781310838574341943");

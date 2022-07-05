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
    /// COEFF_A = 5
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "5");

    /// COEFF_D = 34773060642897309185384047147212537017610844748483156782131927863984860828477
    #[rustfmt::skip]
    const COEFF_D: Fq = field_new!(Fq, "34773060642897309185384047147212537017610844748483156782131927863984860828477");

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
    /// COEFF_A = 17165960291975010705235411328759697642550494794523912163236795778327111240517
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "17165960291975010705235411328759697642550494794523912163236795778327111240517");
    /// COEFF_B = 19487116272480608366017516318857500436152885692725866948687484806255689119093
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "19487116272480608366017516318857500436152885692725866948687484806255689119093");

    type TEModelParameters = EdwardsParameters;
}

/// GENERATOR_X =
/// 9295807447791800517238823544536036265553075291649599896413230022371226008850,
#[rustfmt::skip]
const GENERATOR_X: Fq = field_new!(Fq, "9295807447791800517238823544536036265553075291649599896413230022371226008850");

/// GENERATOR_Y =
/// 37003816485745428581305808186739338174930352395372557733329896099230702862213
#[rustfmt::skip]
const GENERATOR_Y: Fq = field_new!(Fq, "37003816485745428581305808186739338174930352395372557733329896099230702862213");

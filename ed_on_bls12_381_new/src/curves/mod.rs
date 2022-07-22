use crate::{fq::Fq, fr::Fr};
use ark_ec::{
    models::{ModelParameters, MontgomeryModelParameters, SWModelParameters, TEModelParameters},
    short_weierstrass_jacobian::{
        GroupAffine as SWGroupAffine, GroupProjective as SWGroupProjective,
    },
    twisted_edwards_extended::{
        GroupAffine as TEGroupAffine, GroupProjective as TEGroupProjective,
    },
};
use ark_ff::{field_new, Field};

#[cfg(test)]
mod tests;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl TEModelParameters for Parameters {
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
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (GENERATOR_X_TE, GENERATOR_Y_TE);

    type MontgomeryModelParameters = Parameters;

    /// Multiplication by `a` is a multiplication by 5.
    #[inline(always)]
    fn mul_by_a(elem: &Self::BaseField) -> Self::BaseField {
        // return 5*elem
        let t = (*elem).double().double();
        t + *elem
    }
}

impl MontgomeryModelParameters for Parameters {
    /// COEFF_A = 17165960291975010705235411328759697642550494794523912163236795778327111240517
    #[rustfmt::skip]
    const COEFF_A: Fq = field_new!(Fq, "17165960291975010705235411328759697642550494794523912163236795778327111240517");
    /// COEFF_B = 19487116272480608366017516318857500436152885692725866948687484806255689119093
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "19487116272480608366017516318857500436152885692725866948687484806255689119093");

    type TEModelParameters = Parameters;
}

/// GENERATOR_X =
/// 34942408837291627131850307694793404208146687472160864369664796157969165422594,
#[rustfmt::skip]
const GENERATOR_X_TE: Fq = field_new!(Fq, "34942408837291627131850307694793404208146687472160864369664796157969165422594");

/// GENERATOR_Y =
/// 22037139198196189744061797094951336215019119199014308314900632152169958192711
#[rustfmt::skip]
const GENERATOR_Y_TE: Fq = field_new!(Fq, "22037139198196189744061797094951336215019119199014308314900632152169958192711");

pub type EdwardsAffine = TEGroupAffine<Parameters>;
pub type EdwardsProjective = TEGroupProjective<Parameters>;

impl SWModelParameters for Parameters {
    /// COEFF_A = 40134810535214015562426085132763902269106966834552711290100314126475615509598
    #[rustfmt::skip]
    const COEFF_A: Self::BaseField = field_new!(Fq, "40134810535214015562426085132763902269106966834552711290100314126475615509598");

    /// COEFF_B = 175774110750
    #[rustfmt::skip]
    const COEFF_B: Self::BaseField = field_new!(Fq, "175774110750");

    /// COFACTOR = 36
    const COFACTOR: &'static [u64] = &[36];

    /// COFACTOR_INV =
    /// 340650398061230070360097944799693614941776844001335578757751983284292286373
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "340650398061230070360097944799693614941776844001335578757751983284292286373");

    /// generators
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (SW_GENERATOR_X, SW_GENERATOR_Y);
}

/// x coordinate for SW curve generator
#[rustfmt::skip]
const SW_GENERATOR_X: Fq = field_new!(Fq, "30128492855277688777774666107163987310911435419803862567854522187347283372844");
/// y coordinate for SW curve generator
#[rustfmt::skip]
const SW_GENERATOR_Y: Fq = field_new!(Fq, "14111996890051343555183053767536232189942220629775509090241379435415936553243");

pub type WeierstrassAffine = SWGroupAffine<Parameters>;
pub type WeierstrassProjective = SWGroupProjective<Parameters>;

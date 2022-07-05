use crate::*;
use ark_ec::{
    bls12,
    models::{ModelParameters, SWModelParameters},
};
use ark_ff::{field_new, Zero};

pub type G2Affine = bls12::G2Affine<crate::Parameters>;
pub type G2Projective = bls12::G2Projective<crate::Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq2;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = [0, 0]
    const COEFF_A: Fq2 = field_new!(Fq2, g1::Parameters::COEFF_A, g1::Parameters::COEFF_A,);

    /// COEFF_B = [4, 4]
    const COEFF_B: Fq2 = field_new!(Fq2, field_new!(Fq, "4"), field_new!(Fq, "4"),);

    /// COFACTOR = (x^8 - 4 x^7 + 5 x^6) - (4 x^4 + 6 x^3 - 4 x^2 - 4 x + 13) //
    /// 9
    /// = 178978112966391770429872195706951844718855093683853815132669241970945030313539039243852192479088181513529506102583837701698829784604046161619421017145344
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0,
        3959960058809024512,
        3868807880377338447,
        8159717461059949524,
        2630335575209973182,
        2790321835755086778,
        9925587310278742238,
        246241851161470288,
    ];

    /// COFACTOR_INV = COFACTOR^{-1} mod r
    /// 4253360287974064064798190029116424659670136428995308158976
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "4253360287974064064798190029116424659670136428995308158976");

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G2_GENERATOR_X, G2_GENERATOR_Y);

    #[inline(always)]
    fn mul_by_a(_: &Self::BaseField) -> Self::BaseField {
        Self::BaseField::zero()
    }
}

pub const G2_GENERATOR_X: Fq2 = field_new!(Fq2, G2_GENERATOR_X_C0, G2_GENERATOR_X_C1);
pub const G2_GENERATOR_Y: Fq2 = field_new!(Fq2, G2_GENERATOR_Y_C0, G2_GENERATOR_Y_C1);

/// G2_GENERATOR_X_C0 =
/// 
#[rustfmt::skip]
pub const G2_GENERATOR_X_C0: Fq = field_new!(Fq, "50982252389493889162459597758708479251691331029909271737498927304370980709602102405003867391462221286185892534321");

/// G2_GENERATOR_X_C1 =
/// 
#[rustfmt::skip]
pub const G2_GENERATOR_X_C1: Fq = field_new!(Fq, "2258348114731798521869794199456239591704040441144853687693867419021637369725086717615784088967786555913704996922744");

/// G2_GENERATOR_Y_C0 =
/// 
#[rustfmt::skip]
pub const G2_GENERATOR_Y_C0: Fq = field_new!(Fq, "525343502615244661244814896046845139133127875453377780456307641823424754652740084022269150942457018476160458752204");

/// G2_GENERATOR_Y_C1 =
/// 
#[rustfmt::skip]
pub const G2_GENERATOR_Y_C1: Fq = field_new!(Fq, "1414287582683958192890307486407331938007213566421862462518404587110509444027534643370749141796697875971599304365724");

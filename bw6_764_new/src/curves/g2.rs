use crate::{Fq, Fr};
use ark_ec::{
    models::{ModelParameters, SWModelParameters},
    short_weierstrass_jacobian::{GroupAffine, GroupProjective},
};
use ark_ff::field_new;

pub type G2Affine = GroupAffine<Parameters>;
pub type G2Projective = GroupProjective<Parameters>;

#[derive(Clone, Default, PartialEq, Eq)]
pub struct Parameters;

impl ModelParameters for Parameters {
    type BaseField = Fq;
    type ScalarField = Fr;
}

impl SWModelParameters for Parameters {
    /// COEFF_A = 0
    #[rustfmt::skip]

    const COEFF_A: Fq = field_new!(Fq, "0");

    /// COEFF_B = 33
    #[rustfmt::skip]
    const COEFF_B: Fq = field_new!(Fq, "33");

    /// COFACTOR =
    /// 18761113507437584042257724496253649777533321532752457817124580676172013756405106010407665665625358696877349224316932
    #[rustfmt::skip]
    const COFACTOR: &'static [u64] = &[
        0x11b4f40000000004,
        0x453d26ac55830003,
        0x446c8631e74ed9c4,
        0xa32d3a4151208aab,
        0xb20c7f1dfc1d3110,
        0x79e4b6c96f069a04,
    ];

    /// COFACTOR^(-1) mod r =
    /// 1806924286967672791790853093197086714448634152435377887925946384806443737024864304406311021150563015501426854433846
    #[rustfmt::skip]
    const COFACTOR_INV: Fr = field_new!(Fr, "1806924286967672791790853093197086714448634152435377887925946384806443737024864304406311021150563015501426854433846");

    /// AFFINE_GENERATOR_COEFFS = (G2_GENERATOR_X, G2_GENERATOR_Y)
    const AFFINE_GENERATOR_COEFFS: (Self::BaseField, Self::BaseField) =
        (G2_GENERATOR_X, G2_GENERATOR_Y);
    #[inline(always)]
    fn mul_by_a(_elem: &Self::BaseField) -> Self::BaseField {
        use ark_ff::Zero;
        Self::BaseField::zero()
    }
}

/// G2_GENERATOR_X =
///  32806761409557102591379598737679074210716782605601785122059330810275104689749196541937484598367091158662899828703454236057761867122256159543783736879638435670643899329840485490918194462378411770782836648378626733763994955375693991
#[rustfmt::skip]
pub const G2_GENERATOR_X: Fq = field_new!(Fq, "32806761409557102591379598737679074210716782605601785122059330810275104689749196541937484598367091158662899828703454236057761867122256159543783736879638435670643899329840485490918194462378411770782836648378626733763994955375693991");

/// G2_GENERATOR_Y =
/// 12469658574253575017909577836891119467273972982135905168677807358487709981345581401157678619445614381879133735935613607027838030930639414345191163594365610340570582206626931855990841948102610636010279968194038829194016548048547034
#[rustfmt::skip]
pub const G2_GENERATOR_Y: Fq = field_new!(Fq, "12469658574253575017909577836891119467273972982135905168677807358487709981345581401157678619445614381879133735935613607027838030930639414345191163594365610340570582206626931855990841948102610636010279968194038829194016548048547034");
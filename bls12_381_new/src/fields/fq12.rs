use crate::*;
use ark_ff::{field_new, fields::*};

pub type Fq12 = Fp12<Fq12Parameters>;

#[derive(Clone, Copy)]
pub struct Fq12Parameters;

impl Fp12Parameters for Fq12Parameters {
    type Fp6Params = Fq6Parameters;

    // this is a non square defining Fq12/Fq6, we need to of the form (0,1,0)
    // because of the arkworks implementation
    const NONRESIDUE: Fq6 = field_new!(Fq6, FQ2_ZERO, FQ2_ONE, FQ2_ZERO);

    const FROBENIUS_COEFF_FP12_C1: &'static [Fq2] = &[
	// Fq6::NONRESIDUE^(((q^0) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "1"),
		field_new!(Fq, "0"),
	),
	// Fq6::NONRESIDUE^(((q^1) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "787812864132008876187301536602647758945697705514609586754304230397064637312127160804683108864889847739453371361110"),
		field_new!(Fq, "2625484985882892369756883848699037746042915392632002187047211476716175291792621678941867203297351388971936398497269"),
	),
	// Fq6::NONRESIDUE^(((q^2) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "568069564169735887784523595428185157672868461154544401167503000216125933363689723611179842011138"),
		field_new!(Fq, "0"),
	),
	// Fq6::NONRESIDUE^(((q^3) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "2310190043747686757723452389548667764938844165403269247017817432463971276564206670176476175000405973734897206649578"),
		field_new!(Fq, "2565870820107789363084764694247461385228436189225828333529938981797673561842946466066400080023606034820186459588247"),
	),
	// Fq6::NONRESIDUE^(((q^4) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "568069564169735887784523595428185157672868461154544401167503000216125933363689723611179842011137"),
		field_new!(Fq, "0"),
	),
	// Fq6::NONRESIDUE^(((q^5) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "1522377179615677881536150852946020005993146459888659660263513202066906639252079509371793066135516125995443835288468"),
		field_new!(Fq, "2620544906715980428179585586800260416449343297808368899995884972024947874118262764750954379148805424662760043245731"),
	),
	// Fq6::NONRESIDUE^(((q^6) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154752"),
		field_new!(Fq, "0"),
	),
	// Fq6::NONRESIDUE^(((q^7) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "1892346208359074558664403204649189018318124795699933166758853236546384966755810816821738393557660931075056610793643"),
		field_new!(Fq, "54674086608191065094820892552799031220907108582540566465945990227274312275316298684554299125199389842573583657484"),
	),
	// Fq6::NONRESIDUE^(((q^8) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "2680159072491083434283635177082100889479298905786357595840289005788905202900434977410295569058861055203330140143615"),
		field_new!(Fq, "0"),
	),
	// Fq6::NONRESIDUE^(((q^9) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "369969028743396677128252351703169012324978335811273506495340034479478327503731307449945327422144805079612775505175"),
		field_new!(Fq, "114288252383294071766940047004375392035386311988714419983218485145776042224991511560021422398944743994323522566506"),
	),
	// Fq6::NONRESIDUE^(((q^10) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "2680159072491083434283635177082100889479298905786357595840289005788905202900434977410295569058861055203330140143616"),
		field_new!(Fq, "0"),
	),
	// Fq6::NONRESIDUE^(((q^11) - 1) / 6)
	field_new!(Fq2,
		field_new!(Fq, "1157781892875405553315553888305816771270676041325883093249644264876542964815858468254628436287034652819066146866285"),
		field_new!(Fq, "59614165775103006672119154451576360814479203406173853517272494918501729949675212875467123273745354151749938909022"),
	),
    ];

    // /// Multiply this element by the nonresidue 2+3u+2u².
    // fn mul_fp6_by_nonresidue(fe: &Fq6) -> Fq6 {

    // 	Self::NONRESIDUE * fe
    // 	// // (x+yu + zu²) * (2 + 3u + 2u²) = 2x + 3xu + 2xu² + 2yu + 3yu² + 2yu³ +
    // 2zu² + 3zu³ + 2zu³ 	// //                               =
    // (2x+(2y+5z)(1+v)) + (3x+2y)u + (2x+3y+2z)u²     // let mut copy = *fe;
    //     // let x = copy.c0;
    //     // let y = copy.c1;
    // 	// let z = copy.c2;
    // 	// let x_2 = x.double();
    // 	// let x_3 = x_2 + x;
    // 	// let y_2 = y.double();
    // 	// let y_3 = y_2 + y;
    // 	// let z_2 = z.double();
    // 	// let z_5 = z_2.double() + z_2;

    //     // copy.c0 = x_2 + Fq6Parameters::mul_fp2_by_nonresidue(&(y_2 + z_5));
    //     // copy.c1 = x_3+y_2;
    // 	// copy.c2 = x_2 + y_3 + z_2;
    //     // copy
    // }
}

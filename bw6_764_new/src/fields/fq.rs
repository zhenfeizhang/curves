use ark_ff::{
    biginteger::BigInteger768 as BigInteger,
    field_new,
    fields::{FftParameters, Fp768, Fp768Parameters, FpParameters},
};

pub type Fq = Fp768<FqParameters>;

pub struct FqParameters;

pub const FQ_ONE: Fq = field_new!(Fq, "1");
pub const FQ_ZERO: Fq = field_new!(Fq, "0");

impl Fp768Parameters for FqParameters {}
impl FftParameters for FqParameters {
    type BigInt = BigInteger;

    // The internal representation of this type is six 64-bit unsigned
    // integers in little-endian order. Values are always in
    // Montgomery form; i.e., Scalar(a) = aR mod p, with R=2^768.

    // (MODULUS - 1) % 2^TWO_ADICITY == 0
    const TWO_ADICITY: u32 = 42;

    // least_quadratic_nonresidue(MODULUS) in Sage.
    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0x34d1251ae4c6e958,
        0x90b838003f5b02ba,
        0xa98534c0333ba476,
        0xd47e38c61dafe67e,
        0x466f2e4d6dc1dbbd,
        0xae06cb442a9ef009,
        0xe23b98ecaf348c9c,
        0x4671664ceabe1b9f,
        0xdbe8b3c55bce4eba,
        0x215edd8dbcb41192,
        0x73802395ac032fa2,
        0x435ddf72cdccea5,
    ]);
}
impl FpParameters for FqParameters {
    /// MODULUS = 50282768576993852407494634308294841376983574838385858405447848583376065247544506093833501024756151257915195555055355085817082514603850271637792434072458297604453662538937558570183541064674323908560052492735693432884208755873415169
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        0x11b4f40000000001,
        0x4e95441902ad0003,
        0x3d48fc68c9ad78b2,
        0x3e6bf53730ee9d44,
        0xa9d8e02cbf4c19c9,
        0x3cdd20964e1fdd79,
        0xb91795604c2b9b7,
        0x6f212dea07a8c442,
        0xaabc6593e038d2f,
        0xffccb9dc13b94472,
        0x27a2704282a96665,
        0x84a92725c09dcd1,
    ]);

    const MODULUS_BITS: u32 = 764;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    // gap to 64-bit machine word
    const REPR_SHAVE_BITS: u32 = 4;

    // 2^768 % MODULUS
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        0xeccb67ffffffffe2,
        0xca820511afb9ffa3,
        0xd1726bb85dabdb1a,
        0xaf59438844099200,
        0x1895bac19514fa6a,
        0xde162e62d8440bbe,
        0xa4f3c7eb712e3c86,
        0xfa1c9e931a390042,
        0xbfdec18abb957470,
        0x6023835b049faa2,
        0x5af6d834b026000c,
        0x742d69936d81f7d,
    ]);

    // R^2
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        0xb8731bc33c03493a,
        0xc6ac4a080e781037,
        0xcf70f390401f22b8,
        0x455a47059dfcafc7,
        0x3cc913f48ab99a13,
        0x1644c2619f95d399,
        0x6eea9ea70ba1fd57,
        0x3cf13c58f43d96b0,
        0xdc7d9feb66251161,
        0xe887026f893b0aa0,
        0xa8005abd80710073,
        0x3367bf33eb33768,
    ]);

    // (-1/MODULUS) % 2^64
    const INV: u64 = 0x11b4f3ffffffffff;

    /// GENERATOR = 2
    // primitive_root(MODULUS)
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0xb24a6ffffffffecc,
        0x746a11e8c7dbfc4e,
        0x443051ed5b4ac979,
        0xe61cf99920eac9e6,
        0xa7124a29d870f9e0,
        0xc5f4cb2a01a9899f,
        0x14fa048245b88f8a,
        0x4c14c26ec8f3e08a,
        0x29555ca163ba22ee,
        0x3db063384519a6cb,
        0x5090effacc30cd48,
        0x647ce694422545c
    ]);

    // (MODULUS - 1) / 2
    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x88da7a0000000000,
        0x274aa20c81568001,
        0x1ea47e3464d6bc59,
        0x9f35fa9b98774ea2,
        0xd4ec70165fa60ce4,
        0x9e6e904b270feebc,
        0x5c8bcab02615cdb,
        0xb79096f503d46221,
        0x555e32c9f01c697,
        0xffe65cee09dca239,
        0x93d138214154b332,
        0x42549392e04ee68
    ]);

    // T =
    // 3445725192157866269698394841137828771239834456268075054756895080104811711121745868043841591644705843820432283876893306725580879560277123879674755849562650799475802549689254425186271815711798397975949850214984556421382456559534149
    // (MODULUS - 1) / 2 ^ TWO_ADICITY
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        0x1902ad000311b4f4,
        0x68c9ad78b24e9544,
        0x3730ee9d443d48fc,
        0x2cbf4c19c93e6bf5,
        0x964e1fdd79a9d8e0,
        0x5604c2b9b73cdd20,
        0xea07a8c4420b9179,
        0x593e038d2f6f212d,
        0xdc13b944720aabc6,
        0x4282a96665ffccb9,
        0x725c09dcd127a270,
        0x84a92
    ]);

    // (T - 1)/2 =
    // 1722862596078933134849197420568914385619917228134037527378447540052405855560872934021920795822352921910216141938446653362790439780138561939837377924781325399737901274844627212593135907855899198987974925107492278210691228279767074
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xc8156800188da79,
        0x3464d6bc59274aa2,
        0x9b98774ea21ea47e,
        0x165fa60ce49f35fa,
        0x4b270feebcd4ec70,
        0xab02615cdb9e6e90,
        0xf503d4622105c8bc,
        0x2c9f01c697b79096,
        0xee09dca2390555e3,
        0x214154b332ffe65c,
        0x392e04ee6893d138,
        0x42549
    ]);
}

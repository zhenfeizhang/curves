use ark_ff::{
    biginteger::BigInteger384 as BigInteger,
    field_new,
    fields::{FftParameters, Fp384, Fp384Parameters, FpParameters},
};

pub type Fq = Fp384<FqParameters>;

pub struct FqParameters;

impl Fp384Parameters for FqParameters {}
impl FftParameters for FqParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 40;

    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
            0x5880362220b84861,
            0x4bbd0863f45e2f5a,
            0x567955d27df9ef11,
            0xba116084ee9312a1,
            0xbbd0755e36ce3a39,
            0x7cb263c5a186e35,
    ]);
}
impl FpParameters for FqParameters {
    /// MODULUS = 2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154753
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        0xc46d3d0000000001,
        0xa03d26ac55830000,
        0xee0452c2f61f8a4e,
        0x4b1f5637e48acf6a,
        0xed78ad8849344ac4,
        0x1169d0f8346ea849,
    ]);

    const MODULUS_BITS: u32 = 381;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 3;

    /// R = 1879779181519311124355173722617898923386224253461848118764088867037427314546078924644365220969204737403488240140274
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        0x4206a9fffffffff2,
        0x3ca7e29352d5fff5,
        0xfbc379568a466fb3,
        0xe44948f18068a826,
        0x366828bff23e943,
        0xc36926d21f2cbf5,
    ]);

    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        0x5993dfaf9da689f,
        0x7149d8d38ca1fa2a,
        0x826beb11aa94a09e,
        0x4991ec52865a4a39,
        0xf4b92bd510dcd4b5,
        0xcd13c944264e05,
    ]);

    const INV: u64 = 0xc46d3cffffffffff;

    /// GENERATOR = 10
    /// Encoded in Montgomery form,so the value is
    /// 10 * R % q = ?
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0x3545f8ffffffff6d,
        0xfce2cb0ae5c6ff8f,
        0x53847a0cabe394d9,
        0xdd017de7c44ae599,
        0xa3b45abdf6f91148,
        0x3d0179e4755d8c,
    ]);

    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x62369e8000000000,
        0x501e93562ac18000,
        0x770229617b0fc527,
        0x258fab1bf24567b5,
        0xf6bc56c4249a2562,
        0x8b4e87c1a375424,
    ]);

    /// T and T_MINUS_ONE_DIV_TWO,where MODULUS - 1 = 2^S * T
    /// For T coprime to 2
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        0xac55830000c46d3d,
        0xc2f61f8a4ea03d26,
        0x37e48acf6aee0452,
        0x8849344ac44b1f56,
        0xf8346ea849ed78ad,
        0x1169d0,
    ]);

    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x562ac1800062369e,
        0x617b0fc527501e93,
        0x1bf24567b5770229,
        0xc4249a2562258fab,
        0x7c1a375424f6bc56,
        0x8b4e8
    ]);
}

pub const FQ_ONE: Fq = field_new!(Fq, "1");
pub const FQ_ZERO: Fq = field_new!(Fq, "0");

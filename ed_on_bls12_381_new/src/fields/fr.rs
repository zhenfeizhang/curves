use ark_ff::{
    biginteger::BigInteger256 as BigInteger,
    fields::{FftParameters, Fp256, Fp256Parameters, FpParameters},
};

pub type Fr = Fp256<FrParameters>;

pub struct FrParameters;

impl Fp256Parameters for FrParameters {}
impl FftParameters for FrParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 3;

    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0x122e33eb18afce23,
        0xe7c9d224a7fe0668,
        0xdb89145ad85fa08a,
        0x10840f3a46a122,
    ]);
}
impl FpParameters for FrParameters {
    /// MODULUS = 1114855848200389321178502364798997285263996944004370985025370127112229300857
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        0x908d8791068f2a79,
        0xda3a46492f0a8b72,
        0x6ff7b76e90541b7,
        0x276fc6db157e275
    ]);

    const MODULUS_BITS: u32 = 250;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 6;

    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        0xd70e74a65c65e951,
        0x328db88e12c1e4e7,
        0x2f3553283ee28f07,
        0x2206fdda5a3e2ea,
    ]);

    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        0x9fbfffb8f1773e50,
        0x98d74f96dfd044fb,
        0x3fc9e4e2d1f0476c,
        0x4c799f652dfd88,
    ]);

    const INV: u64 = 0xc9164276ddb4e037;

    //
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0xe22451f76781c962,
        0x27b70343433e9579,
        0xa01963db2caf8889,
        0x18c7b3aeda7c97d,
    ]);

    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x4846c3c88347953c,
        0xed1d2324978545b9,
        0x837fbdbb7482a0db,
        0x13b7e36d8abf13a,
    ]);

    const T: BigInteger = BigInteger([
        0x5211b0f220d1e54f,
        0xfb4748c925e1516e,
        0xa0dfef6edd20a836,
        0x4edf8db62afc4e,
    ]);

    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x2908d8791068f2a7,
        0x7da3a46492f0a8b7,
        0x506ff7b76e90541b,
        0x276fc6db157e27,
    ]);
}

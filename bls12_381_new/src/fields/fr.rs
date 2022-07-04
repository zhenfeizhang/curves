use ark_ff::{
    biginteger::BigInteger256 as BigInteger,
    fields::{FftParameters, Fp256, Fp256Parameters, FpParameters},
};

pub type Fr = Fp256<FrParameters>;

pub struct FrParameters;

impl Fp256Parameters for FrParameters {}
impl FftParameters for FrParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 32;

    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xb9b58d8c5f0e466a,
        0x5b1b4c801819d7ec,
        0xaf53ae352a31e64,
        0x5bf3adda19e9b27b,
    ]);
}
impl FpParameters for FrParameters {
    /// MODULUS = 40134810535214015562426085132763902269106966834552711290100314126475667177473
    #[rustfmt::skip]
    const MODULUS: BigInteger = BigInteger([
        0x88da7a0000000001,
        0x8595441902ad0001,
        0xfbed5cb8c4bd3dd9,
        0x58bb7f6cf05bd874,
    ]);

    const MODULUS_BITS: u32 = 255;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 1;

    /// R = 35522468166888164298718814743160103315056050996535141459256955754961795284990
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        0xee4b0bfffffffffe,
        0xf4d577cdfaa5fffc,
        0x825468e7685844c,
        0x4e8901261f484f16,
    ]);
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        0x545614b750ae0708,
        0x941d7ccdec477e60,
        0x3bea247108f3fc34,
        0x22a93fbb087a705a,
    ]);

    const INV: u64 = 0x88da79ffffffffff;

    /// GENERATOR = 11
    /// Encoded in Montgomery form,so the value here is
    /// 11 * R % q =
    /// 29533855018843667224072195979886016043653859450912154440923686166298743537633
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0x6d8b39ffffffffe1,
        0xd2ecc0f8ad0cffd0,
        0x7e41c5a02d1582a8,
        0x414b91cee4e0c9d5,
    ]);

    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xc46d3d0000000000,
        0xc2caa20c81568000,
        0x7df6ae5c625e9eec,
        0x2c5dbfb6782dec3a,
    ]);

    // T and T_MINUS_ONE_DIV_TWO,where MODULUS - 1 = 2^S * T
    // For T coprime to 2

    // T = (MODULUS - 1) / 2^S =
    // 18251198769218724720497487002268782029708275710869525614771334461
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        0xc81568000c46d3d,
        0x5c625e9eecc2caa2,
        0xb6782dec3a7df6ae,
        0x2c5dbf,
    ]);

    // (T - 1) / 2 =
    // 6104339283789297388802252303364915521546564123189034618274734669823
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x640ab400062369e,
        0x2e312f4f76616551,
        0xdb3c16f61d3efb57,
        0x162edf,
    ]);
}

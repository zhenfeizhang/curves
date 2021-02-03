use ark_ff::{
    biginteger::BigInteger256 as BigInteger,
    fields::{FftParameters, Fp256, Fp256Parameters, FpParameters},
};

pub struct FrParameters;

pub type Fr = Fp256<FrParameters>;

impl Fp256Parameters for FrParameters {}
impl FftParameters for FrParameters {
    type BigInt = BigInteger;

    const TWO_ADICITY: u32 = 6;

    // TWO_ADIC_ROOT_OF_UNITY = GENERATOR^T
    // Encoded in Montgomery form, so the value here is (5^T)R mod q.
    // 96880290535369799104859127461068109275753111126600080047934318051043430856655
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xe823f5b94e5b77cf,
        0xcab86f88ecd9ebea,
        0xd3d93fa52862700f,
        0xd6304d1f577eeabe,
    ]);
}

impl FpParameters for FrParameters {
    // 115792089237316195423570985008687907852837564279074904382605163141518161494337
    const MODULUS: BigInteger = BigInteger([
        0xbfd25e8cd0364141,
        0xbaaedce6af48a03b,
        0xfffffffffffffffe,
        0xffffffffffffffff,
    ]);

    // R = 2^256 mod q = 432420386565659656852420866394968145599
    const R: BigInteger = BigInteger([
        0x402da1732fc9bebf,
        0x4551231950b75fc4,
        0x1,
        0x0,
    ]);

    // R2 = (2^256)^2 mod q
    const R2: BigInteger = BigInteger([
        0x896cf21467d7d140,
        0x741496c20e7cf878,
        0xe697f5e45bcd07c6,
        0x9d671cd581c69bc5,
    ]);

    // 57896044618658097711785492504343953926418782139537452191302581570759080747168
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0xdfe92f46681b20a0,
        0x5d576e7357a4501d,
        0xffffffffffffffff,
        0x7fffffffffffffff,
    ]);

    // T and T_MINUS_ONE_DIV_TWO, where MODULUS - 1 = 2^S * T

    // T = 1809251394333065553493296640760748560200586941860545380978205674086221273349
    const T: BigInteger = BigInteger([
        0xeeff497a3340d905,
        0xfaeabb739abd2280,
        0xffffffffffffffff,
        0x3ffffffffffffff,
    ]);

    // (T - 1) / 2 = 904625697166532776746648320380374280100293470930272690489102837043110636674
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x777fa4bd19a06c82,
        0xfd755db9cd5e9140,
        0xffffffffffffffff,
        0x1ffffffffffffff,
    ]);

    // GENERATOR = 5
    // Encoded in Montgomery form, so the value here is 5R mod q.
    // 2162101932828298284262104331974840727995
    const GENERATOR: BigInteger = BigInteger([
        0x40e4273feef0b9bb,
        0x5a95af7e9394ded5,
        0x6,
        0x0,
    ]);

    const MODULUS_BITS: u32 = 256;

    const CAPACITY: u32 = Self::MODULUS_BITS - 1;

    const REPR_SHAVE_BITS: u32 = 1;

    // INV = -q^{-1} (mod 2^64)
    const INV: u64 = 5408259542528602431;
}

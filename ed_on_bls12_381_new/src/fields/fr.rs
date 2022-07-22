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
    // 2^3-th root of unity = GENERATOR ^ T =
    // 29180994090598693311887686510943151134939632683795886146282286790774607395
    // represented in Montgomery representation
    // GENERATOR ^ T * R % q =
    // 184876279233171354607023484706228756414603643739074403084986775692900093042
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0x774dab031cfccc72,
        0x34fefb80d7f23bde,
        0x3497dcc3ceccba84,
        0x68a2e2520ae6c9,
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

    // R = 961936872676095342185241434391187471078299433190352581844460915353511651665
    #[rustfmt::skip]
    const R: BigInteger = BigInteger([
        0xd70e74a65c65e951,
        0x328db88e12c1e4e7,
        0x2f3553283ee28f07,
        0x2206fdda5a3e2ea,
    ]);

    // R2 = R^2 =
    // 135119785518099739587603646013875297644837167035153369810067176280917753424
    #[rustfmt::skip]
    const R2: BigInteger = BigInteger([
        0x9fbfffb8f1773e50,
        0x98d74f96dfd044fb,
        0x3fc9e4e2d1f0476c,
        0x4c799f652dfd88,
    ]);

    // INV = -1/2⁶⁴ mod q
    const INV: u64 = 0xc9164276ddb4e037;

    // GENERATOR = 10 is represented in Montgomery form
    // 10 * R % q =
    // 700521941157838852424395425519896428671018779868557938241648136637282109794
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0xe22451f76781c962,
        0x27b70343433e9579,
        0xa01963db2caf8889,
        0x18c7b3aeda7c97d,
    ]);

    // (r-1)/2 = 557427924100194660589251182399498642631998472002185492512685063556114650428
    #[rustfmt::skip]
    const MODULUS_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x4846c3c88347953c,
        0xed1d2324978545b9,
        0x837fbdbb7482a0db,
        0x13b7e36d8abf13a,
    ]);

    // T = (r-1) / 2^TWO_ADICITY =
    // 139356981025048665147312795599874660657999618000546373128171265889028662607
    const T: BigInteger = BigInteger([
        0x5211b0f220d1e54f,
        0xfb4748c925e1516e,
        0xa0dfef6edd20a836,
        0x4edf8db62afc4e,
    ]);

    // (T-1) / 2 =
    // 69678490512524332573656397799937330328999809000273186564085632944514331303
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x2908d8791068f2a7,
        0x7da3a46492f0a8b7,
        0x506ff7b76e90541b,
        0x276fc6db157e27,
    ]);
}

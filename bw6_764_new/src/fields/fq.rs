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

    // generator ^ T =
    // 15647041070701591247141755234906323016646441133306316182450461109722262761590257721038054015949296206968562679473488875746562490498055080246726201579898724026586191924329707670487880219686982518755872834545250256634025261882689989
    // represented in Montgomery form
    // generator ^ T * R % q =
    // 29194214453274612788259896948757470482017543557522301991176235505209529475970809598802347474678483182646008383158008074304412382759704804596581792182723014305914493089396569394954185099754896933188917348925674861171398468109761762
    #[rustfmt::skip]
    const TWO_ADIC_ROOT_OF_UNITY: BigInteger = BigInteger([
        0xb91aa557204e7ce2,
        0x4fdaa74e408afbc5,
        0x89c8788abc51af93,
        0x2546c47f82cb9bee,
        0x6434a826d3712d01,
        0x4c6094e639ccc339,
        0x66759c7de9989e5e,
        0x86ea8c8e367e511e,
        0xab23ef6019c366,
        0xc6234560f21e3838,
        0x7efc746c04af7cbe,
        0x4d05df2a34e8474,
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

    // R = 2^768 % MODULUS =
    // 44035034990893362924140459213657313947378771965120858975616580524768995260041703515873378085961940213031864045470420631659104605999306242310514252867432211070845099852723148444759053691368727568461424411645665764366584177613602786
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

    // R2 = R^2 =
    // 19484276896059200577738625846660958282422134027159736868748903233768699782662318678346332295878460929511096670476323228208093923156943116229451886818685416686513883849813970763678367267226825376965967521616474642181180351827888442
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

    /// GENERATOR = 11
    // Finding a generator is expensive with this size of p
    // We use the following trick:
    // ````
    // from sage.rings.factorint import factor_trial_division
    // partial_facto = factor_trial_division(p-1, 1<<20)
    // def is_generator(g):
    //     for (f,_) in partial_facto:
    //         if g**((p-1)//f) == 1:
    //             return False
    //     return True
    // g = Fp(1)
    // while not(is_generator(g)):
    //     g +=1
    // # g = Fp(11)
    // ```
    // g is represented in Montgomery form
    // g*R%q = 31840467706882320498093342575576881028314318070856723082751748522074360632558183830105649722775981022113744504676431175896408034557716220675524874889629643339213135529516605760697721022987088076035196093481082512074547150888894125
    #[rustfmt::skip]
    const GENERATOR: BigInteger = BigInteger([
        0x8d60e3fffffffead,
        0xf056d2e174e8fbef,
        0xd859c13cef492be1,
        0x570a47ea3405bea2,
        0x15cf24beae39da82,
        0x672dd8f68bcdb7e4,
        0xae5c5317b224125a,
        0xd7103317db841c8a,
        0xde8857d2e14c0a2f,
        0x43e5e191e1aa5cfb,
        0x83e557ecf9ad66ee,
        0x54012901ef09708,
    ]);

    // (MODULUS - 1) / 2 =
    // 25141384288496926203747317154147420688491787419192929202723924291688032623772253046916750512378075628957597777527677542908541257301925135818896217036229148802226831269468779285091770532337161954280026246367846716442104377936707584
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

    // T = 11432977902812547746974504277271726409022352263124471029515882169875127430610633858539927365372057296989440955043245386563531272850889370519733446161003097951027862686473226533549573347198548596807098077246267492429117
    // (MODULUS - 1) / 2 ^ TWO_ADICITY
    #[rustfmt::skip]
    const T: BigInteger = BigInteger([
        0x640ab4000c46d3d,
        0x1a326b5e2c93a551,
        0x4dcc3ba7510f523f,
        0xb2fd306724f9afd,
        0x259387f75e6a7638,
        0x558130ae6dcf3748,
        0x7a81ea311082e45e,
        0x964f80e34bdbc84b,
        0x7704ee511c82aaf1,
        0x10a0aa59997ff32e,
        0x9c9702773449e89c,
        0x212a4,
    ]);

    // (T - 1)/2 =
    // 5716488951406273873487252138635863204511176131562235514757941084937563715305316929269963682686028648494720477521622693281765636425444685259866723080501548975513931343236613266774786673599274298403549038623133746214558
    #[rustfmt::skip]
    const T_MINUS_ONE_DIV_TWO: BigInteger = BigInteger([
        0x832055a00062369e,
        0x8d1935af1649d2a8,
        0xa6e61dd3a887a91f,
        0x597e9833927cd7e,
        0x12c9c3fbaf353b1c,
        0x2ac0985736e79ba4,
        0xbd40f5188841722f,
        0xcb27c071a5ede425,
        0x3b8277288e415578,
        0x850552cccbff997,
        0x4e4b813b9a24f44e,
        0x10952,
    ]);
}

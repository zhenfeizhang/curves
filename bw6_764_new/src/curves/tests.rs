use ark_ec::{bw6::BW6Parameters, AffineCurve, PairingEngine, ProjectiveCurve};
use ark_ff::{field_new, Field, FpParameters, One, PrimeField, UniformRand, Zero};
use ark_std::{rand::Rng, test_rng};

use crate::*;

use ark_algebra_test_templates::{curves::*, groups::*};

#[test]
fn test_fq3_frob() {
    let mut rng = ark_std::test_rng();
    let x = Fq3::rand(&mut rng);
    let x_p = x.pow(FqParameters::MODULUS);
    let mut x_frob = x;
    x_frob.frobenius_map(1);
    assert_eq!(x_frob, x_p);
}

#[test]
fn test_fq6_frob() {
    let mut rng = ark_std::test_rng();
    let x = Fq6::rand(&mut rng);
    let mut x_p = x.pow(FqParameters::MODULUS);
    for power in 1..6 {
        let mut x_pi = x;
        x_pi.frobenius_map(power);
        assert_eq!(x_pi, x_p);
        x_p = x_p.pow(FqParameters::MODULUS);
    }
}

#[test]
fn test_g1_projective_curve() {
    curve_tests::<G1Projective>();

    sw_tests::<g1::Parameters>();
}

#[test]
fn test_g1_projective_group() {
    let mut rng = test_rng();
    let a: G1Projective = rng.gen();
    let b: G1Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g1_generator() {
    let generator = G1Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_g2_projective_curve() {
    curve_tests::<G2Projective>();

    sw_tests::<g2::Parameters>();
}

#[test]
fn test_g2_projective_group() {
    let mut rng = test_rng();
    let a: G2Projective = rng.gen();
    let b: G2Projective = rng.gen();
    group_test(a, b);
}

#[test]
fn test_g2_generator() {
    let generator = G2Affine::prime_subgroup_generator();
    assert!(generator.is_on_curve());
    assert!(generator.is_in_correct_subgroup_assuming_on_curve());
}

#[test]
fn test_rand() {
    let mut rng = test_rng();
    let x: G1Projective = rng.gen();
    let y = x.mul(FrParameters::MODULUS);
    assert!(y.is_zero());
}

#[test]
fn test_mul_x() {
    // a is in the cyclotomic subgroup
    let a = field_new!(
        Fq6,
        field_new!(
            Fq3,
            field_new!(
                Fq,
                "956515994761003844815699113074682888127888356183844506446578619\
    1545684216529830458964344021920203297490418273395910947806906023949040497429820\
    1718773745094537913615276315683985258533126302729871747417776042048257471714209\
    38121372"
            ),
            field_new!(
                Fq,
                "178636476345923624916512774936771098737256534570295046320101389\
    5647235033454324794251903683089538511985043070913629744601711053998641936247223\
    2069945284322301064579096695326228201972417196393013908217075915927102640639123\
    015126336"
            ),
            field_new!(
                Fq,
                "234429345755413483874207698256701019133127699645905808605555096\
    4323838495852642664278747305020004356032296679433041419841036177565876757303267\
    2095120243445680968848312088593826097400053601081382527548405863036380879156350\
    839890318"
            ),
        ),
        field_new!(
            Fq3,
            field_new!(
                Fq,
                "165039401514749120564330154840138160319559740767704767319682387\
    5461538840856221451066065154395206858553588146875292127744750586026837348750768\
    5834543361249327757153254959187046665324421761499326254149305704922457430599317\
    814548249"
            ),
            field_new!(
                Fq,
                "285875065142912894259859683218012181594483975562838813773538354\
    0040773387024997346546553516916692521344797464814729496001208219602696168357254\
    3351188221946039916261995050555045888049596482697517391694307689352037400079722\
    059918554"
            ),
            field_new!(
                Fq,
                "212414982278109706120284242380323723964988731149526885896945999\
    8085900972295351702167094310517840533653758478116188196722208147655439338482581\
    1414561579319073578420895950190811219190531173392920703841133158421252599077143\
    364228923"
            ),
        ),
    );
    let a_x = a.pow(Parameters::X);
    let a_x_2 = a.cyclotomic_exp(&Parameters::X);
    assert_eq!(a_x, a_x_2);
}

#[test]
fn test_bilinearity() {
    let mut rng = test_rng();
    // let a: G1Projective = rng.gen();
    // let b: G2Projective = rng.gen();
    let a = G1Projective::prime_subgroup_generator();
    let b = G2Projective::prime_subgroup_generator();
    let s: Fr = rng.gen();

    let sa = a.mul(s.into_repr());
    let sb = b.mul(s.into_repr());

    let ans1 = BW6_764New::pairing(sa, b);
    let ans2 = BW6_764New::pairing(a, sb);
    let ans3 = BW6_764New::pairing(a, b).pow(s.into_repr());

    assert_eq!(ans1, ans2);
    assert_eq!(ans2, ans3);

    assert_ne!(ans1, Fq6::one());
    assert_ne!(ans2, Fq6::one());
    assert_ne!(ans3, Fq6::one());

    assert_eq!(ans1.pow(Fr::characteristic()), Fq6::one());
    assert_eq!(ans2.pow(Fr::characteristic()), Fq6::one());
    assert_eq!(ans3.pow(Fr::characteristic()), Fq6::one());
}

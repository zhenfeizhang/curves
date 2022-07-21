C.<x> = QQ[]
main_q_x = (x**6 - 2*x**5 + 2 * x**3+x+1)/3
main_r_x = x**4-x**2+1
Qd.<d> = QQ.extension(main_q_x)
Qdt.<t> = Qd[]

u=0x8508c00000000001

[(ζ0, _), (ζ1, _)] = Qdt(cyclotomic_polynomial(6)).roots()

t_0_x = C((ζ0+1).list())
t_1_x = C((ζ1+1).list())

sqrtm3 = Qd(-3).sqrt()

y_0_x = C(((ζ0-1)/sqrtm3).list())
y_1_x = C(((ζ1-1)/sqrtm3).list())

t0 = t_0_x(u)
t1 = t_1_x(u)
y0 = y_0_x(u)
y1 = y_1_x(u)

# BW6 curve

r = ZZ(main_q_x(u))

h_t=13
h_y=9

t = t0 + h_t * r
y = y0 + h_y * r
q = ZZ((t**2 + 3*y**2)/4)

Fq = GF(q)
b = Fq(4)
E = EllipticCurve([0,b])
assert E.order() % r == 0

# we are in the case of t_{bw, 3} and y_{bw, 3} in the notations of Aurore and Youssef.
assert t0 == C((ζ0+1).list())(u)
assert y0 == t0//3


def final_exp_hard_bw6_bls12_trace_3_mod_r_mod_u_subfunction(B, u, ht, hy):
    """
    return B^((ht^2+3*hy^2)/4*r + 3*((ht+hy)/2*t0/3 + (d-1)/3) + 1)

    INPUT:
    - `B`: a finite field element, with B.conjugate() available (absolute extension of a prime field)
    - `u`: relative integer, the curve seed
    - `ht`: relative integer, curve parameter, lifting cofactor for the trace t0
    - `hy`: relative integer, curve parameter, lifting cofactor for the square part y in t0^2-4*q = -D*y^2

    RETURN: B^e

    parameters are
    r       = (u-1)^2/3*(u^4-u^2+1) + u
    r      == (u^6 - 2*u^5 + 2*u^3 + u + 1)/3
    d       = u^4 - 4*u^3 + 7*u^2 - 6*u + 3
    (d-1)/3 = (u - 1)/3 * (u-1) * ((u-1)^2 + 1)
    t3      = u^5 - 3*u^4 + 3*u^3 - u + 3
    t3/3   == (u-1)^2/3*(u^3-u^2+1) + (u-1)/3 + 1

    (u^3-u^2+1) == (u+1)*(u^2-2*u+2) - 1

    a sequence to obtain the exponents r, t3/3 and (d-1)/3 is
    C = (u-1)/3
    D = (u-1)*C
    D == (u-1)^2/3
    E = D * ((u-1)^2 + 1)
    E == (d-1)/3
    D = -D
    F = D+1
    G = (u+1) * E + F
    G == (u-1)^2/3*(u^3-u^2+1) + 1
    H = G + C
    H == t3/3
    I = (u+1)*(G+D) - F
    I == r

    cost e((u-1)/3) + 3e(u-1) + 2e(u+1) + e(d1) + e(d2) + 10 M + S + 2cj
    """
    # cost e((u-1)/3) + 3e(u-1) + 2e(u+1) + e(d2) + e(d1) + 10M + S + 2 cj
    C = B**((u-1)//3)
    D = C**(u-1)
    E = (D**(u-1))**(u-1) * D             # B^((d-1)/3)
    D = D.conjugate()
    Fc = D * B
    G = E**(u+1) * Fc
    H = G * C                             # B^(t3/3)
    I = (G * D)**(u+1) * Fc.conjugate()   # B^r
    d2 = (ht**2+3*hy**2)//4
    d1 = (ht+hy)//2
    J = H**d1 * E
    K = J**2 * J * B * I**d2
    return K

def final_exp_hard_bw6_bls12_trace_3_mod_r_mod_u(m, u, ht, hy):
    """
    Exponentiation to (u+1)*Phi_k(p(u))/r(u)

    e0 = (ht^2+3*hy^2)/4*r*(u^3-u^2-u) + (ht+hy)/2*tr3*(u^3-u^2-u) + cc3*(u^3-u^2-u) + 3*(u^2-2*u+1)
    e1 = (ht^2+3*hy^2)/4*r*(u+1) + (ht+hy)/2*tr3*(u+1) + cc3*(u+1) + 3

    ((u^3-u^2-u) + (u+1)*q)*((ht^2+3*hy^2)/4*r + (ht+hy)/2*tr3 + cc3) + 3*((u^2-2*u+1) + q)
    with (u^3-u^2-u) + (u+1)*q == (u+1)(u^2-2*u+1 + q) - 1
    ((u+1)(u^2-2*u+1 + q) - 1)*((ht^2+3*hy^2)/4*r + 3*((ht+hy)/2*tr3/3 + (cc3-1)/3) + 1) + 3*(u^2-2*u+1 + q)

    cost subfunction e((u-1)/3) + 3e(u-1) + 2e(u+1) + e(d2) + e(d1) + 10M +  S       + 2cj
    cost here                     2e(u-1) +  e(u+1)                 +  4M +  S + frb +  cj
    cost total       e((u-1)/3) + 5e(u-1) + 3e(u+1) + e(d1) + e(d2) + 14M + 2S + frb + 3cj
    """
    # cost 2e(u-1) + e(u+1) + 4M + S + frb + cj
    A = m**(u-1)
    A = A**(u-1)
    A = A * m.frobenius()        # A = m^((u-1)^2 + q)
    B = A**(u+1) * m.conjugate() # B = m**((u^3-u^2-u) + (u+1)*q)
    A = A**2 * A                 # A = m**(3*((u^2-2*u+1) + q))

    C = final_exp_hard_bw6_bls12_trace_3_mod_r_mod_u_subfunction(B, u, ht, hy)
    return A * C

proof.arithmetic(False)
Fqx.<xx> = Fq[]
Fq6.<v> = GF(q**6, modulus = xx**6 +4)
X = 4706060808928810110243033687222149646557764975833824608134322922776699677088976909328419929113709204911027092541524539114933080650613132515455714023263397679906765746332495711140960432813274564817284872791282909247687145743030426*v^5 + 934763988504458968416866483830767200440768549105935908126093184344502193528229715307609518083720159724523416890725398775926934723033657404671571090148801041952812496446835277652485790568321664561025957891997443848056482914342492*v^4 + 4055211677718714100122613644151112900990365549873157651183539782683042330695350198644020024764441836647960983638066047843192607847680521921207665774937036709410449775592343091058315466150441475189192412263456814369779599357736855*v^3 + 5883808584483885977273006247255940900200000693779883620167146932170447939583401122740304651932527546092011608726884733872500637882371421404602625349472200600798971896063108176471262311196857409521809151925438535515209700738901570*v^2 + 6038489901735106239999317818778609527546619809349000202612657965445163910916806271105240018445887540898468964678518582889038691602509839444497521575714846148517149431540187821529423729559304109587013362449304064589555765981831586*v + 4299888316588777934288677787492042065147644974669011243874219449599160408676313088194520559231032583459529214743124049338101193565783106955603002157185962718901046466048082667909980448884614192110353772550673901281035430195956179

def print_for_rust(A) :
    [a0,a1,a2,a3,a4,a5] = A.polynomial().list()

    print("field_new!(Fq6, field_new!(Fq3,")
    print("field_new!(Fq, \"{}\"),".format(a0))
    print("field_new!(Fq, \"{}\"),".format(a2))
    print("field_new!(Fq, \"{}\"),".format(a4))
    print("),")
    print("field_new!(Fq3, ")
    print("field_new!(Fq, \"{}\"),".format(a1))
    print("field_new!(Fq, \"{}\"),".format(a3))
    print("field_new!(Fq, \"{}\"),".format(a5))
    print("),")
    print(");")
'''
a0+a1u+a2u² + (b0+b1u+b2u²)v
a0 + a1u+a2u² + (b0+b1u+b2u²)sqrt(u)
a0 + a1 α^1/3 + a2 α^2/3 + (b0 + b1 α^1/3 + b2 α^2/3) α^1/6
a0 α^0/6 + a1 α^2/6 + a2 α^4/6 + b0 α^1/6 + b1 α^3/6 + b2 α^5/6

a0 α^0/6 + a1 α^1/6 + a2 α^2/6 + a3 α^3/6 + a4 α^4/6 + a5 α^5/6


'''


pow_r = Fq6.random_element()**r
Y = X**((q**6-1)//r) * pow_r
# Y is in the cyclotomic subgroup


# assert final_exp_hard_bw6_bls12_trace_3_mod_r_mod_u(Y, u, h_t, h_y) == Y**((u+1) * cyclotomic_polynomial(6)(q) // r)
print('let Y = ')
Y = Y**((q**6-1)/cyclotomic_polynomial(6)(q))
print_for_rust(Y)
print('let out = ')
print_for_rust(Y**((u+1) * cyclotomic_polynomial(6)(q) // r))
# print("let y_u1 = ")
# print_for_rust(Y**(u))
# print("let y_u2 = ")
# print_for_rust(Y**(u-1))
# print("let y_u3 = ")
# print_for_rust(Y**((u-1)//3))

# print("let y_conj = ")
# print_for_rust(Y.conjugate())
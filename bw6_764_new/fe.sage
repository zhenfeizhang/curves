C.<x> = QQ[]
main_q_x = (x**6 - 2*x**5 + 2 * x**3+x+1)/3
main_r_x = x**4-x**2+1
Qd.<d> = QQ.extension(main_q_x)
Qdt.<t> = Qd[]

u=0xc46d3d0000000001

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

h_t=-4
h_y=-2

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
Fq6.<v> = GF(q**6, modulus = xx**6 -11)
X = 13710830679704876750318201381243717374981411135961643025722722679911480932348703762656019929170852282010315964985861632726879967921163932282933529419923679064892585615377209789023901162841505830558955337702671680955090708388614862*v^5 + 46797255432776212480593804409565915924635922682960104935757424834733831385201296679205345676870593309204190009687230085402256496316396602707610431532532931870844717906465805674023690090136422508637547611423446646099507555360978998*v^4 + 26739885678555334125755255217710303730933719607803424886176783259196938426587230314750260130291249171412650111992054687460689460484738936579650029920872992438815726348851571572235188066815652562817134500344925260146458427480360144*v^3 + 3841775257568792723084146439529413457263403618805517230970140277183942002145384544587951907951157117648631744299942095060090622017522848847101890317940466451021899825986576759837368802140627778227072287921329298869461333236770142*v^2 + 13877918637442104997707882352463625441843060028139480618551244988912802349637669076849473911720379101147555202000554355521151977669138349378061763874885545654805944882538249408354635905743291720297663747825711408108309088390930829*v + 15682868650744327587600531785440186264949041658276423979034566651374492100597056545682191809930830711335652617789319556064086218604383239452885688356889237643238801066517996140856742120995071061824271859203799444757518801840099222

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


print('let y = ')
YY = Y**((q**6-1)/cyclotomic_polynomial(6)(q))
print_for_rust(Y)
print('let out = ')
print_for_rust(YY**((u+1) * cyclotomic_polynomial(6)(q) // r))

# assert final_exp_hard_bw6_bls12_trace_3_mod_r_mod_u(Y, u, h_t, h_y) == Y**((u+1) * cyclotomic_polynomial(6)(q) // r)


# print('let out = ')
# print_for_rust(Y**((u+1) * cyclotomic_polynomial(6)(q) // r))

# print("let y_u1 = ")
# print_for_rust(Y**(u))
# print("let y_u2 = ")
# print_for_rust(Y**(u-1))
# print("let y_u3 = ")
# print_for_rust(Y**((u-1)//3))

# print("let y_conj = ")
# print_for_rust(Y.conjugate())
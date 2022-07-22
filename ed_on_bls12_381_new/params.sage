p = 40134810535214015562426085132763902269106966834552711290100314126475667177473
r = 1114855848200389321178502364798997285263996944004370985025370127112229300857

def print64(x):
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(hex(y%(1<<64))))
        y >>= 64
    print(']')

Fr = GF(r)
g = Fr(10)
while g.multiplicative_order() != r-1:
    g += 1

two_adicity = valuation(r - 1, 2);
trace = (r - 1) / 2**two_adicity;
two_adic_root_of_unity = g**trace
M = 1<<64
while M < r :
    M *= 1<<64
R = Fr(M)

# weierstrass
Fp = GF(p)
Aw = Fp(40134810535214015562426085132763902269106966834552711290100314126475615509598)
Bw = Fp(175774110750)
E  = EllipticCurve([Aw,Bw])
assert E.order() % r == 0
P = 36*E.random_point()
assert not(P.is_zero())

# Montgomery
α = E.division_polynomial(2).roots()[0][0]
Fpx.<x> = Fp[]
s = 1/(x**2-(3*α**2+Aw)).roots()[1][0]
Am = 3*α*s
Bm = s

assert Aw == (3-Am**2) / (3*Bm**2)
assert Bw == (2*Am**3 - 9*Am) / (27*Bm**3)

[u,v] = [s*(P[0] - α), s*P[1]]
assert Bm * v**2 == u**3 + Am*u**2 + u

# Twisted Edwards
Ate0 = (Am+2)/Bm
Dte0 = (Am-2)/Bm
[xte0,yte0] = [u/v, (u-1)/(u+1)]
assert Ate0 * xte0**2 + yte0**2 == 1 + Dte0*xte0**2*yte0**2

β = Fp(1)
boo = True
while β.is_square():
    β *= -1
    if not(boo):
        β += 1
    boo = not(boo)
# β = Fp(5)

# isomorphism (sqrt(Ate0/-5)x, y)
Ate = β*Ate0/Ate0
Dte = β*Dte0/Ate0
xte = xte0 *sqrt(Ate0/β)
yte = yte0
assert Ate * xte**2 + yte**2 == 1 + Dte*xte**2*yte**2

new_Am = 2*(Ate + Dte) / (Ate - Dte)
new_Bm = 4 / (Ate-Dte)

cof = 36
cof_inv = Fr(36)**-1

Fp2.<u> = GF(p**2)
E2 = E.change_ring(Fp2)

# GLV endomorphism (WIP)

for (x7, _) in E2.division_polynomial(7).roots():
    P7 = E2.lift_x(x7)
    assert 7*P7 == 0 and P7 != 0
    if E2.isogeny_codomain(P7).j_invariant() == E2.j_invariant():
        break

(φ,π) = E2.isogeny(P7).rational_maps()
φn,φd = φ.numerator(), φ.denominator()
πn,πd = π.numerator(), π.denominator()

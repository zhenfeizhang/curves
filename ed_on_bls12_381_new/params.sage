p = 40134810535214015562426085132763902269106966834552711290100314126475667177473
r = 1114855848200389321178502364798997285263996944004370985025370127112229300857

print(valuation(p-1,2))
print(valuation(r-1,2))

def print64(x):
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(hex(y%(1<<64))))
        y >>= 64
    print(']')

Fr = GF(r)
# Fr
# generator = Fr(0);
# for i in range(0, 20):
#     i = Fr(i);
#     neg_i = Fr(-i)
#     if not(i.is_primitive_root() or neg_i.is_primitive_root()):
#         continue
#     elif i.is_primitive_root():
#         assert(i.is_primitive_root());
#         print("Generator: %d" % i)
#         generator = i
#         break
#     else:
#         assert(neg_i.is_primitive_root());
#         print("Generator: %d" % neg_i)
#         generator = neg_i
#         break
generator = Fr(10)

two_adicity = valuation(r - 1, 2);
trace = (r - 1) / 2**two_adicity;
two_adic_root_of_unity = generator^trace
print("2-adic Root of Unity: %d " % two_adic_root_of_unity)
print("TWOADICROOTOFUNITY")
print64(two_adic_root_of_unity)
print("MODULUS")
print64(r)
print("MODULUSBITS")
print(r.nbits())
M = 1<<64
while M < r :
    M *= 1<<64
R = Fr(M)
print("R")
print64(R)
print("R2")
print64(R**2)
print("INV")
print64(-Integers(1<<64)(r)**-1)
print("GENERATOR")
print64((generator*R)%r)
print("MODULUS_MINUS_ONE_DIV_TWO")
print64((r-1)//2)
print("T")
print64(trace)
print("T_MINUS_ONE_DIV_TWO")
print64((trace-1)//2)


# weierstrass
Fp = GF(p)
Aw = Fp(40134810535214015562426085132763902269106966834552711290100314126475615509598)
Bw = Fp(175774110750)
E  = EllipticCurve([Aw,Bw])
assert E.order() % r == 0
P = 36 * E.random_point()
assert not(P.is_zero())

# Montgomery
α = E.division_polynomial(2).roots()[0][0]
Fpx.<x> = Fp[]
s = 1/(x**2-(3*α**2+Aw)).roots()[0][0]
Am = 3*α*s
Bm = s

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

# isomorphism (sqrt(ate0/5)x, y)
Ate = 5*Ate0/Ate0
Dte = 5*Dte0/Ate0
xte = xte0 *sqrt(Ate0/5)
yte = yte0
assert Ate * xte**2 + yte**2 == 1 + Dte*xte**2*yte**2
print("TE COEFFICIENTS")
print(Ate)
print(Dte)

print("MONTGOMERY COEFFICIENTS")
print(Am)
print(Bm)

cof = 36
cof_inv = Fr(36)**-1
print("COFACTOR_INV")
print(cof_inv)
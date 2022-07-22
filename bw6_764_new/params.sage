p = 50282768576993852407494634308294841376983574838385858405447848583376065247544506093833501024756151257915195555055355085817082514603850271637792434072458297604453662538937558570183541064674323908560052492735693432884208755873415169
r = 2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154753
val2p = valuation(p - 1, 2)

proof.arithmetic(False)

Fp = GF(p)
E = EllipticCurve(Fp, [0, 3])
assert E.order()%r == 0

def print64(x):
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(hex(y%(1<<64))))
        y >>= 64
    print(']')

from sage.rings.factorint import factor_trial_division
partial_facto = factor_trial_division(p-1, 1<<20)
def is_generator(g):
    for (f,_) in partial_facto:
        if g**((p-1)//f) == 1:
            return False
    return True
g = Fp(1)
while not(is_generator(g)):
    g +=1
# g = Fp(11)
two_adicity = valuation(p - 1, 2);
trace = (p - 1) / 2**two_adicity;
two_adic_root_of_unity = g^trace
M = 1<<64
while M < p :
    M *= 1<<64
R = Fp(M)
T = (p-1)//(1<<two_adicity)

# Fq3
# we choose α=11, see magma code for details
α = Fp(11)

# α = Fp(1)
# Fpx.<x> = Fp[]
# while not((x**3 - α).is_irreducible()):
#     α += 1

T = (p**3-1)//(1<<valuation(p**3-1,2))

Fpx.<x> = Fp[]
Fp3.<u> = GF(p**3, modulus = x**3-α)
Fp3y.<y> = Fp3[]

for j in range(3):
    print(u**((p**(3*j)-1)//3))
for j in range(3):
    print(u**((2*p**(3*j)-1)//3))

# we define Fp6 using `0,1,0`, see magma code and akrworks implem for details.
nsq = u
assert (y**3-nsq).is_irreducible()

# G1
Fr = GF(r)
cof = E.order()//r

# # G2
# Fpx.<x> = Fp[]
# α = Fp(1)
# boo = True
# while not((x**6-α).is_irreducible()):
#     α = -α
#     if boo:
#         α+=1
#     boo = not(boo)
# E2 = EllipticCurve([0, E.a6() * α])
# assert E2.order()%r == 0
# cof2 = E2.order()//r
# print("COF2")
# print(cof2)
# inv_cof2 = 1/GF(r)(cof2)
# print("INV_COF2")
# print(inv_cof2)

# t = cof2
# while t>0 :
#     print(t%(1<<64))
#     t >>= 64



def naf(E):
    Z = []
    while E>0:
        if E%2 == 1:
            Z.append(2-E%4)
            E = E-Z[-1]
        else:
            Z.append(0)
        E = E//2
    return Z

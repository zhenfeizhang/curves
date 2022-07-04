from re import I, U
from sympy import true


p = 50282768576993852407494634308294841376983574838385858405447848583376065247544506093833501024756151257915195555055355085817082514603850271637792434072458297604453662538937558570183541064674323908560052492735693432884208755873415169
r = 2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154753
val2p = valuation(p - 1, 2)
val2r = valuation(r - 1, 2)

proof.arithmetic(False)

Fp = GF(p)
E = EllipticCurve(Fp, [0, 3])
assert E.order()%r == 0


# non square
β = Fp(-11)
Fpx.<x> = Fp[]
Fp2.<i> = GF(p**2, modulus = x**2+β)

E2 = EllipticCurve(Fp, [0,3*11])
# E2 : y² = x³+33

def print64(x):
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(hex(y%(1<<64))))
        y >>= 64
    print(']')

generator = Fp(2)
#check?

two_adicity = valuation(p - 1, 2);
trace = (p - 1) / 2**two_adicity;
two_adic_root_of_unity = generator^trace
print("2-adic Root of Unity: %d " % two_adic_root_of_unity)
print("TWOADICROOTOFUNITY")
print64(two_adic_root_of_unity)
print("MODULUS")
print64(p)
print("MODULUSBITS")
print(p.nbits())
M = 1<<64
while M < p :
    M *= 1<<64
R = Fp(M)
print("R")
print64(R)
print("R2")
print64(R**2)
print("INV")
print64(-Integers(1<<64)(p)**-1)
print("GENERATOR")
print64((10*R)%p)
print("MODULUS_MINUS_ONE_DIV_TWO")
print64((p-1)//2)
print("T")
T = (p-1)//(1<<40)
print64(T)
print("T_MINUS_ONE_DIV_TWO")
print64((T-1)//2)


α = Fp(1)
boo = True
while not((x**3 - α).is_irreducible()):
    if boo:
        α+=1
    else :
        α*= -1
    boo = not(boo)
# alpha = 2

for i in range(3):
    print(α**((p**i-1)//3))
for i in range(3):
    print(α**((2*p**i-1)//3))

Fp3.<u> = GF(p**3, modulus = x**3-α)
nsq = Fp3(1)
cpt = 0
while nsq.is_square():
    if cpt == 0 :
        nsq +=u 
    if cpt == 1:
        nsq+=u**2
    if cpt == 3:
        nsq+=1
    cpt+=1
    cpt%=3
print(nsq)


# G1
Fr = GF(r)
cof = E.order()//r
print("COFACTOR")
print64(cof)
print("COFACTOR_INV")
print(Fr(cof)**-1)

# G2
cof2 = E2.order()//r 
print("COFACTOR")
print(cof2)
print64(cof2)
print(Fr(cof2)**-1)

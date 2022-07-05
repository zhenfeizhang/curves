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
print("2-adic Root of Unity: %d " % two_adic_root_of_unity)
print("TWOADICROOTOFUNITY")
print64(two_adic_root_of_unity*R%p)
print("MODULUS")
print64(p)
print("MODULUSBITS")
print(p.nbits())
print("R")
print64(R)
print("R2")
print64(R**2)
print("INV")
print64(-Integers(1<<64)(p)**-1)
print("GENERATOR")
print64((g*R)%p)
print("MODULUS_MINUS_ONE_DIV_TWO")
print64((p-1)//2)
print("T")
T = (p-1)//(1<<valuation(p-1,2))
print64(T)
print("T_MINUS_ONE_DIV_TWO")
print64((T-1)//2)

# Fq3
print("FQ3")
α = Fp(1)
Fpx.<x> = Fp[]
while not((x**3 - α).is_irreducible()):
    α += 1

ns = Fp(1)
while ns.is_square():
    ns+=1
print(ns)
#ns = Fp(11)

T = (p**3-1)//(1<<valuation(p**3-1,2))
print("T_MINUS_ONE_DIV_2")
print64((T-1)//2)

print("QUADRATIC_NONRESIDUE_TO_T")
print(ns**T)


Fp3.<u> = GF(p**3, modulus = x**3-α)
nsq = Fp3(u)
if nsq.is_square():
    nsq *= ns
# nsq = 11*u

for i in range(3):
    print(nsq**((p**(3*i)-1)//3))
for i in range(3):
    print(nsq**((2*p**(3*i)-1)//3))

#Fq6
print("FQ6")
for i in range(6):
    print(nsq**((p**(3*i)-1)//6))



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

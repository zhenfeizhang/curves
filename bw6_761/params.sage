p = 6891450384315732539396789682275657542479668912536150109513790160209623422243491736087683183289411687640864567753786613451161759120554247759349511699125301598951605099378508850372543631423596795951899700429969112842764913119068299
r = 258664426012969094010652733694893533536393512754914660539884262666720468348340822774968888139573360124440321458177
val2p = valuation(p - 1, 2)

proof.arithmetic(False)

Fp = GF(p)
E = EllipticCurve(Fp, [0, -1])
assert E.order()%r == 0

def print64(x):
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(hex(y%(1<<64))))
        y >>= 64
    print(']')

def print10(x):
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(y%(1<<64)))
        y >>= 64
    print(']')

from sage.rings.factorint import factor_trial_division
partial_facto = factor_trial_division(p-1, 1<<20)
def is_generator(g):
    for (f,_) in partial_facto:
        if g**f == 1:
            return False
    return True
g = Fp(1)
while not(is_generator(g)):
    g +=1
# g = Fp(2)
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
print10(R)
print("R2")
print10(R**2)
print("INV")
print10(-Integers(1<<64)(p)**-1)
print("GENERATOR")
print10((g*R)%p)
print("MODULUS_MINUS_ONE_DIV_TWO")
print64((p-1)//2)
print("T")
T = (p-1)//(1<<valuation(p-1,2))
print64(T)
print("T_MINUS_ONE_DIV_TWO")
print64((T-1)//2)

# Fq3
print("FQ3")
α = Fp(-1)
Fpx.<x> = Fp[]
while not((x**3 - α).is_irreducible()):
    α -= 1
α = Fp(-4) #idk why they don't use a smaller??

T = (p**3-1)//(1<<valuation(p**3-1,2))
print("T_MINUS_ONE_DIV_2")
print64((T-1)//2)

print("QUADRATIC_NONRESIDUE_TO_T")
print(α**T)


Fp3.<u> = GF(p**3, modulus = x**3-α)
nsq = Fp3(u)
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
# nsq = u

for i in range(3):
    print(nsq**((p**(3*i)-1)//3))
for i in range(3):
    print(nsq**((2*p**(3*i)-1)//3))

#Fq6
print("FQ6")
for i in range(6):
    print(nsq**((p**(3*i)-1)//6))



# # G1
# Fr = GF(r)
# cof = E.order()//r
# print("COFACTOR")
# print64(cof)
# print("COFACTOR_INV")
# print(Fr(cof)**-1)

# # G2
# cof2 = E2.order()//r 
# print("COFACTOR")
# print(cof2)
# print64(cof2)
# print(Fr(cof2)**-1)

C.<x> = QQ[]
main_q_x = (x**6 - 2*x**5 + 2 * x**3+x+1)/3
main_r_x = x**4-x**2+1

u = (main_q_x - r).roots()[0][0]

u2 = u**3 - u**2 - u
t=ZZ(u2)
while t>0:
    print(t%(1<<8))
    t>>=8

L = [
    -1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 1, 0, 0, 1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 0, -1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1,
    0, 0, 1, 0, 1, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, -1, 0, -1, 0, 0, 0, 0, -1, 0, 0,
    1, 0, 0, 0, -1, 0, 0, -1, 0, 1, 0, -1, 0, 0, 0, 1, 0, 0, 1, 0, -1, 0, 1, 0, 1, 0, 0, 0, 1,
    0, -1, 0, -1, 0, 0, 0, 0, 0, 1, 0, 0, 1,
]

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
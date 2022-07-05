from re import I


p = 2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154753
r = 40134810535214015562426085132763902269106966834552711290100314126475667177473

assert p.is_prime()
assert r.is_prime()
proof.arithmetic(False)

# G1
Fp = GF(p)
E = EllipticCurve(Fp, [0,1])
assert E.order()%r == 0

# G2
Fpx.<x> = Fp[]
Fp2.<i> = GF(p**2, modulus=x**2+5)

b2 = 1+i
boo = True
E2 = EllipticCurve(Fp2, [0,b2])
while E2.order()%r != 0:
    if boo:
        b2 += i
    else:
        b2 += 1
    boo = not(boo)
    E2 = EllipticCurve(Fp2, [0,b2])
# b2 = 4+4i


def print64(x):
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(hex(y%(1<<64))))
        y >>= 64
    print(']')

# Fp
g = Fp(1)
while g.multiplicative_order() != p-1:
    g +=1
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
print64((int(g)*int(R))%p)
print("MODULUS_MINUS_ONE_DIV_TWO")
print64((p-1)//2)
print("T")
print64(trace)
print("T_MINUS_ONE_DIV_TWO")
print64((trace-1)//2)

# Fp6
print("FROBENIUS_C1")
for j in range(6):
    print((1+i)**((p**j-1)//3))
print("FROBENIUS_C2")
for j in range(6): 
    print((1+i)**((2*p**j-1)//3))

# Fp12
# todo

print("\n\n\n\n\n\n\n\n")
# Fr
N = 1<<64
while N < r:
    N *= 1<<64
RR = GF(r)(N)
two_adicity = valuation(r - 1, 2);
trace = (r - 1) / 2**two_adicity;
Fr = GF(r)
generator = Fr(39415827777849273770726111108287440543547393994679752525064544064550880998990)
two_adic_root_of_unity = generator^trace
print("2-adic Root of Unity: %d " % two_adic_root_of_unity)
print("TWOADICROOTOFUNITY")
print64(two_adic_root_of_unity*RR%r)
print("MODULUS")
print64(r)
print("R")
print64(RR)
print("R2")
print64(RR**2)
print("INV")
print64(-Integers(1<<64)(r)**-1)
print("GENERATOR")
g = Fr(1)
while g.multiplicative_order()!=r-1:
    g+=1
print64((int(g)*int(RR))%r)
print("MODULUS_MINUS_ONE_DIV_TWO")
print64((r-1)//2)
print("T")
print64(trace)
print("T_MINUS_ONE_DIV_TWO")
print64((trace-1)//2)


x0 = Fp(0)
cof = E.order()//r
P = E(0)
while P.is_zero():
    x0+=1
    while not((x0**3 + 1).is_square()):
        x0+=1
    P = cof * E.lift_x(x0)
print(P[0])
print(-P[1])

x1 = Fp2(0)
boo = True
cof2 = E2.order()//r
Q = E2(0)
while Q.is_zero():
    if boo:
        x1+=i
    else:
        x1+=1
    boo = not(boo)
    while not((x1**3+E2.a6()).is_square()):
        if boo:
            x1+=i
        else:
            x1+=1
        boo = not(boo)
        Q = cof2 * E2.lift_x(x1)

# #params frobenius
# for k in range(6): 
#     T=((3+2*i)**((2*p**k-1)//3)).polynomial().list() 
#     if len(T) == 1: 
#         T+= [0] 
#     print(T[0]) 
#     print(T[1]) 
#     print() 
     
# for k in range(6): 
#     T=((3+2*i)**((p**k-1)//3)).polynomial().list() 
#     if len(T) == 1: 
#         T+= [0] 
#     print(T[0]) 
#     print(T[1]) 
#     print() 

# for k in range(12):
#     print("\t// Fp2::NONRESIDUE^(((q^{}) - 1) / 6)".format(k))
#     T=((3+2*i)**((p**k-1)//6)).polynomial().list() 
#     if len(T) == 1: 
#         T+= [0] 
#     print("\tfield_new!(Fq2, fiend_new!(Fq, \"{}\"), field_new!(Fq, \"{}\"),),".format(
#         T[0],
#         T[1]
#     ))
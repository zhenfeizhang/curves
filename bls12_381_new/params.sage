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
generator = Fp(0);
for i in range(0, 20):
    i = Fp(i);
    neg_i = Fp(-i)
    if not(i.is_primitive_root() or neg_i.is_primitive_root()):
        continue
    elif i.is_primitive_root():
        assert(i.is_primitive_root());
        print("Generator: %d" % i)
        generator = i
        break
    else:
        assert(neg_i.is_primitive_root());
        print("Generator: %d" % neg_i)
        generator = neg_i
        break

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

# Fp6
print("FROBENIUS_C1")
for j in range(6):
    print((1+i)**((p**j-1)//3))
print("FROBENIUS_C2")
for j in range(6): 
    print((1+i)**((2*p**j-1)//3))

# Fp12
# todo

# Fr
N = 1<<64
while N < r:
    N *= 1<<64
RR = GF(r)(N)
print("R")
print64(RR)
print("R2")
print64(RR**2)
print("INV")
print64(-Integers(1<<64)(r)**-1)
Fr = GF(r)
generator = Fr(0);
for i in range(0, 20):
    i = Fr(i);
    neg_i = Fr(-i)
    if not(i.is_primitive_root() or neg_i.is_primitive_root()):
        continue
    elif i.is_primitive_root():
        assert(i.is_primitive_root());
        print("Generator: %d" % i)
        generator = i
        break
    else:
        assert(neg_i.is_primitive_root());
        print("Generator: %d" % neg_i)
        generator = neg_i
        break
print("GENERATOR")
print64((11*RR)%r)
print("MODULUS_MINUS_ONE_DIV_TWO")
print64((r-1)//2)
print("T")
T = (r-1)//(1<<41)
print64(T)
print("T_MINUS_ONE_DIV_TWO")
print64((T-1)//2)
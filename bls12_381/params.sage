r = 52435875175126190479447740508185965837690552500527637822603658699938581184513

assert r.is_prime()
proof.arithmetic(False)

def print64(x):
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(hex(y%(1<<64))))
        y >>= 64
    print(']')

N = 1<<64
while N < r:
    N *= 1<<64
RR = GF(r)(N)
two_adicity = valuation(r - 1, 2);
trace = (r - 1) / 2**two_adicity;
Fr = GF(r)
g = Fr(1)
while g.multiplicative_order()!=r-1:
    g+=1
two_adic_root_of_unity = g^trace
print("2-adic Root of Unity: %d " % two_adic_root_of_unity)
print("TWOADICROOTOFUNITY")
print64(two_adic_root_of_unity)
print("MODULUS")
print64(r)
print("R")
print64(RR)
print("R2")
print64(RR**2)
print("INV")
print64(-Integers(1<<64)(r)**-1)
print("GENERATOR")

print64((int(g)*RR)%r)
print("MODULUS_MINUS_ONE_DIV_TWO")
print64((r-1)//2)
print("T")
T = (r-1)//(1<<valuation(r-1,2))
print64(T)
print("T_MINUS_ONE_DIV_TWO")
print64((T-1)//2)


p = 4002409555221667393417789825735904156556882819939007885332058136124031650490837864442687629129015664037894272559787
Fp = GF(p)
α=-1
Fpx.<x> = Fp[]
Fp2.<u> = GF(p**2, modulus=x**2-α)
nr = 1
boo = True
while nr.is_square():
    if boo:
        nr += u
    else :
        nr+=1
    boo = not(boo)
print(nr)

for k in range(6):
    L = (nr**((p**(k)-1)//3)).polynomial().list()
    x0=L[0]
    if len(L) == 1:
        x1 = 0
    else :
        x1 = L[1]
    print("\t\t// Fp2(u+1)^(((q^{}) - 1) / 3)".format(k))
    print("\t\tfield_new!(Fq2,")
    print("\t\t\tfield_new!(Fq, \"{}\"),".format(x0))
    print("\t\t\tfield_new!(Fq, \"{}\"),".format(x1))
    print("\t\t),")
print()

for k in range(6):
    L = (nr**((2*p**(k)-2)//3)).polynomial().list()
    x0=L[0]
    if len(L) == 1:
        x1 = 0
    else :
        x1 = L[1]
    print("\t\t// Fp2(u+1)^(((2q^{}) - 2) / 3)".format(k))
    print("\t\tfield_new!(Fq2,")
    print("\t\t\tfield_new!(Fq, \"{}\"),".format(x0))
    print("\t\t\tfield_new!(Fq, \"{}\"),".format(x1))
    print("\t\t),")
print()



for k in range(12):
    L = (nr**((p**(k)-1)//6)).polynomial().list()
    x0=L[0]
    if len(L) == 1:
        x1 = 0
    else :
        x1 = L[1]
    print("\t// Fp2::NONRESIDUE^(((q^{}) - 1) / 6)".format(k))
    print("\tfield_new!(Fq2,")
    print("\t\tfield_new!(Fq, \"{}\"),".format(x0))
    print("\t\tfield_new!(Fq, \"{}\"),".format(x1))
    print("\t),")

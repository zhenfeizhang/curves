p = 2680159072491083434851704741251836777263822501214542753513157466943449604067937977626421502422550778814509982154753
r = 40134810535214015562426085132763902269106966834552711290100314126475667177473

assert p.is_prime()
assert r.is_prime()
proof.arithmetic(False)

def print64(x):
    # print a large number in slice of 64 bit integers
    y=int(x)
    print('[')
    while y>0:
        print("\t{}, ".format(hex(y%(1<<64))))
        y >>= 64
    print(']')

# Fp
print('// informations to be written in `fq.rs`')
print('const')

# G1
Fp = GF(p)
E = EllipticCurve(Fp, [0,1])
assert E.order()%r == 0

# G2
Fpx.<x> = Fp[]
Fp2.<i> = GF(p**2, modulus=x**2+5)
Fp2y.<y> = Fp2[]

β = Fp2(1)
boo = True
while not((y^6-β).is_irreducible()):
    if boo:
        β+=i
    else:
        β+=1
    boo = not(boo)
# β = 3+2*u
E2 = EllipticCurve(Fp2, [0,E.a6()/β])
assert E2.order()%r==0
cof2 = E2.order()//r

print("G2 CURVE b coeff")
print(E2.a6())

# generator of G2
x0 = Fp2(0)
boo = True
while not((x0**3 + E2.a6()).is_square()):
    if boo:
        x0 += i
    else : 
        x0 += 1
    boo = not(boo)
G2 = cof2 * E2.lift_x(x0)
print("G2")
print(G2)
print("COF2")
print64(cof2)
print("COF2INV")
print(1/GF(r)(cof2))

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

# Fp2
print("FP2")
α = Fp(-1)
Fpx.<x> = Fp[]
while not((x**2 - α).is_irreducible()):
    α -= 1
# α = -5
for j in range(2):
    print(α**((p**j-1)//2))

# Fq6
print("FQ6")
Fpx.<x> = Fp[]
Fp2.<u> = GF(p**2, modulus = x**2-α)
Fp2y.<y> = Fp2[]
nr = u 
boo = True
while not((y**3-nr).is_irreducible()):
    if boo:
        nr+=1
    else:
        nr += u
    boo = not(boo)

# lets use 3+2u
nr = 3+2*u

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

y_p = 1159217304081619989882107667069130610541357395063680341419086275394355831199957035375337733349481598274022642035670 + i * 976713034202114907481153429861481130818760560976323098692654506896009677833247024916661263028313110749146302147728
x_p = 2596400099176114804917243108478212738621636150343921611194585422446881678592884374934006168171031370044650250085876 + i * 2126850314685004855577289299966814601514376824357723629529864484052710534726599419994568935204535726666526813998074
x_g2 = 2067139113073977370454192518988707467393083928666127211433606788044306124275173068663608018582758826508517402258906 + i * 2519491998099478843702116758769244480142430579776535408270797948761734862573627165832048034395710680584757305104851
y_g2 = 2474906770223728565598559851153671703490955144891383558602760965363069720203249089045277554451635807497922134527093 + i * 263485543252427709458881514577098953824396806081294433627839092712502182178572192733147566767010724429979484635393
p = E2(x_p, y_p)
g2 = E2(x_g2, y_g2)

def print_point(p):
    for coord in p :
        coeffs = coord.polynomial().list()
        for c in coeffs:
            print(hex(c))

t = cof2
while t >0 :
    print(t%(1<<64))
    t >>= 64
    
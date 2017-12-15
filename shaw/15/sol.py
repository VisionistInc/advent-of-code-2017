INPUT_A = 703
INPUT_B = 516

a = INPUT_A
b = INPUT_B

count = 0

for i in range(40000000):
    a *= 16807
    b *= 48271
    a %= 2147483647
    b %= 2147483647

    if (a & 0xFFFF) == (b & 0xFFFF):
        count += 1

print('part 1', count)

a = INPUT_A
b = INPUT_B

count = 0
comps = 0

change_a = True
change_b = True

while comps < 5000000:

    if change_a:
        a *= 16807
        a %= 2147483647
    if change_b:
        b *= 48271
        b %= 2147483647

    if a % 4 == 0:
        change_a = False
    if b % 8 == 0:
        change_b = False

    if (not change_a) and (not change_b):
        change_a = True
        change_b = True
        comps += 1
        if (a & 0xFFFF) == (b & 0xFFFF):
            count += 1

print('part 2', count)
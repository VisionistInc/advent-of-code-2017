
h = 0
for b in range(105700, 122701, 17):
    # no need to go beyond half of b
    for d in range(2, b//2):
        if (b % d) == 0:
            h += 1
            # no need to check for others if we found 1 factor
            break
print(h)
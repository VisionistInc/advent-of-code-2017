INPUT = 314

pos = 0
l = [0]
for i in range(1,2018):
    pos = (pos + INPUT) % len(l) + 1
    l.insert(pos, i)
pos += 1
if pos == len(l):
    pos = 0
print("Part 1", l[pos])

size = 1
pos = 0
val = 1

for i in range(1,50000001):
    pos = (pos + INPUT) % size + 1
    if pos == 1:
        val = size
    size += 1

print("Part 2", val)
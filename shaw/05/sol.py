
with open("input") as f:
    lines = f.readlines()

sol1 = []
sol2 = []

for line in lines:
    sol1.append(int(line))
    sol2.append(int(line))

moves = 0
pos = 0

while pos >= 0 and pos < len(sol1):
    next = pos + sol1[pos]
    sol1[pos] += 1
    pos = next
    moves += 1

print(moves)

moves = 0
pos = 0

while pos >= 0 and pos < len(sol2):
    next = pos + sol2[pos]
    if sol2[pos] < 3:
        sol2[pos] += 1
    else:
        sol2[pos] -= 1
    pos = next
    moves += 1

print(moves)

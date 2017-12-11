with open("input") as f:
    line = f.readline().strip()

x = 0
y = 0
big = 0

moves = line.split(',')

for move in moves:
    if 'n' in move:
        y += 1
    else:
        y -= 1

    if 'e' in move:
        x += 1
    elif 'w' in move:
        x -= 1

    cur = max(abs(x), abs(y))
    big = max(cur, big)

print('part 1', max(abs(x), abs(y)))
print('part 2', big)
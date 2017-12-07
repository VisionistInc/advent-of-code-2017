with open("input") as f:
    lines = f.readlines()

blocks = lines[0].split()
blocks = list(map(int, blocks))

history = []
history.append(list(blocks))

count = 0

while True:
    count += 1

    hi = 0

    # find largest value in list (tie goes to earliest)
    for i in range(len(blocks)):
        if blocks[i] > blocks[hi]:
            hi = i

    blocks_to_spread = blocks[hi]
    blocks[hi] = 0
   
    for i in range(blocks_to_spread):
        hi += 1
        if hi == len(blocks):
            hi = 0
        blocks[hi] += 1
   
    if blocks in history:
        loop_size = count - history.index(blocks)
        break
   
    history.append(list(blocks))

print(count, loop_size)

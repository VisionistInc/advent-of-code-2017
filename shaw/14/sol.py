
INPUT="vbqugkhl"

def knotHash(s):
    # pull out from input
    lens = list(map(ord, s))
    # add in extra input
    lens += [17,31,73,47,23]

    # create two lists like above
    nums = range(256)
    marker = range(256)

    skip = 0

    # do the same loop as above, only 64 times
    for i in range(64):
        for l in lens:
    
            nums_l = nums[:l]
            nums_r = nums[l:]
            nums = list(reversed(nums_l)) + list(nums_r)

            tot = (l + skip) %256

            nums = nums[tot:] + nums[:tot]
            marker = list(marker[tot:]) + list(marker[:tot])

            skip += 1

    # reorder the nums list so index 0 is really the first position
    i = marker.index(0)
    nums = nums[i:] + nums[:i]

    # init our hex string
    b = []
    # our first xor sum can come from pos 0
    xor = nums[0]

    # for pos 1 onwards
    for x in range(1,256):
        # if we've got 16 bytes xor'd
        if x %16 == 0:
            # add to out hex string and reset the xor sum
            b.append(xor)
            xor = 0
        xor = xor ^ nums[x]
    # add in the last xor sum to the hex string
    b.append(xor)

    return b

count = 0
grid = []
for i in range(128):
    b = knotHash(INPUT+'-'+str(i))
    line = ''
    for n in b:
        s = bin(n)[2:]
        s = '0'*(8 - len(s)) + s
        line += s
    count += line.count('1')
    grid.append(list(map(int, line)))

print(count)

groups = 0

done = False
while not done:
    done = True
    for y in range(len(grid)):
        for x in range(len(grid[y])):
            if grid[y][x] == 1:
                done = False
                break
        if not done:
            break
    if done:
        break

    groups += 1

    group = [[y,x]]
    while len(group) > 0:
        (y,x) = group.pop()
        if y > 0 and grid[y-1][x] == 1:
            group.append([y-1,x])
        if y < len(grid) - 1 and grid[y+1][x] == 1:
            group.append([y+1,x])
        if x > 0 and grid[y][x-1] == 1:
            group.append([y,x-1])
        if x < len(grid[y]) - 1 and grid[y][x+1] == 1:
            group.append([y,x+1])
        grid[y][x] = 0

print(groups)

with open("input") as f:
    line = f.readline().strip()

# read in lens, convert to ints
lens = line.split(',')
lens = list(map(int, lens))

# nums holds reverses and everything
# marker will track where first number should be
nums = range(256)
marker = range(256)

skip = 0

# for all lens
for l in lens:
    
    # pull out the portion we will reverse
    nums_l = nums[:l]
    nums_r = nums[l:]
    # create new list with reversed entries
    nums = list(reversed(nums_l)) + nums_r

    # how much total are we skipping (account for going around)
    tot = (l + skip) %256

    # reorder list so it starts with next position
    # change marker to keep track of where index 0 moved
    nums = nums[tot:] + nums[:tot]
    marker = marker[tot:] + marker[:tot]

    skip += 1

# use marker to find real index 0, 1 on nums list
sol = nums[marker.index(0)] * nums[marker.index(1)]
print("part 1",sol)

# pull out from input
lens = list(map(ord, line))
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
        nums = list(reversed(nums_l)) + nums_r

        tot = (l + skip) %256

        nums = nums[tot:] + nums[:tot]
        marker = marker[tot:] + marker[:tot]

        skip += 1

# reorder the nums list so index 0 is really the first position
i = marker.index(0)
nums = nums[i:] + nums[:i]

# init our hex string
s = ''
# our first xor sum can come from pos 0
xor = nums[0]

# for pos 1 onwards
for x in range(1,256):
    # if we've got 16 bytes xor'd
    if x %16 == 0:
        # add to out hex string and reset the xor sum
        s += format(xor, '02x')
        xor = 0
    xor = xor ^ nums[x]
# add in the last xor sum to the hex string
s += format(xor, '02x')

print("part 2", s)
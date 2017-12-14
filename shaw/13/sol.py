import time
# fastFail tells us to bail the first time we are caught
# rather than calculate the risk score
# since there is a 0 entry, the risk score could be 0
# but we would have been caught
def getScore(depths, delay = 0, fastFail = False):
    score = 0
    
    # for each depth in the firewall
    for i in range(len(depths)):
        # if the range is 0, skip
        if depths[i] == 0:
            continue
        # otherwise calculate the number of states for that range
        n = (depths[i] - 1) * 2
        
        # offset the depth by the delay and see if we hit the scanner
        if ((i + delay) % n) == 0:
            # if so add the score
            score += (i * depths[i])
            
            # if we are bailing on first caught, do so
            if fastFail:
                return True
                
    # either return the risk score, or the fact we weren't caught
    if fastFail:
        return False
    return score

with open("input") as f:
    lines = f.readlines()

depths = []

# assumes ordered list for input
for line in lines:
    vals = line.strip().split(': ')
    if len(vals) != 2:
        continue
    vals = list(map(int, vals))
    # if the next depth is a few past the last one
    while len(depths) < vals[0]:
        # append some depths with 0 range
        depths.append(0)
    # append the current range to this depth
    depths.append(vals[1])
        

score = getScore(depths)
print('part 1', score)

delay = 0
caught = True
t = time.time()
while caught:
    delay += 1
    caught = getScore(depths, delay, True)

print('part 2', delay, time.time()-t)


depths = []

# we're going to see if we can speed up part 2 by sorting the list
# and testing the shorter ranges first (the one more likely to get hit)
for line in lines:
    vals = line.strip().split(': ')
    if len(vals) != 2:
        continue
    vals = map(int, vals)
    depths.append(list(vals))
depths.sort(key=lambda x: x[1])

delay = 0
caught = True
t = time.time()
while caught:
    delay += 1
    caught = False
    # for each depth in the firewall
    for vals in depths:
        # calculate the number of states for that range
        n = (vals[1] - 1) * 2
        
        # offset the depth by the delay and see if we hit the scanner
        if ((vals[0] + delay) % n) == 0:
            # if so add the score
            caught = True
            break
    
print('part 2 better', delay, time.time()-t)
orig = ['a','b','c','d','e','f','g','h','i','j','k','l','m','n','o','p']

# this will summarize the totality of all spins and exchanges
# l is the result of a bunch of spins and exchanges
# this returns a list of final positions mapped from
# the original positon
def generateSpinExchangeSum(l):
    res = []
    for c in l:
        res.append(orig.index(c))
    return res

# this will summarize the totality of all partners
# l is the result of a bunch of partners
# this returns a reduced list of partners moves to generate
# the final list from the original list
def generateParternSum(l):
    res = []
    start = list(orig)
    # for everything in l
    for i in range(len(l)):
        # if the values are the same, no need to partner
        if start[i] == l[i]:
            continue
        # otherwise, create new partner from orig list to what exists now
        res.append([start[i], l[i]])
        # update orig list to reflect new state
        start[start.index(l[i])] = start[i]
        start[i] = l[i]
    return res

# helper function for doing all partners
def doPartners(l, moves):
    for move in moves:
        x = l.index(move[0])
        y = l.index(move[1])
        tmp = l[x]
        l[x] = l[y]
        l[y] = tmp

# helper function for doing all spins and exchanges
# moves is a summation of all transitions after a turn(s)
def doSpinExchanges(l, moves):
    res = []
    for i in moves:
        res.append(l[i])
    return res

def printList(l):
    s = ''
    for c in l:
        s += c
    print(s)

with open("input") as f:
    moves = f.readline().strip()

moves = moves.split(',')

spins_exchanges = []
partners = []

# partners can be done independently of spins/exchanges
# so we generate two lists
for move in moves:
    if move[0] == 'p':
        (x,y) = move[1:].split('/')
        partners.append([x,y])
    else:
        spins_exchanges.append(move)

# for the first time through the spins/exchanges, we do all the hard work
d = list(orig)
for move in spins_exchanges:
    if move[0] == 's':
        x = int(move[1:])
        d = d[-x:] + d[:-x]
    else:
        (x,y) = list(map(int, move[1:].split('/')))
        tmp = d[x]
        d[x] = d[y]
        d[y] = tmp

# once we do all the spins/exchanges, we can make a map of orig position to new position
spins_exchanges = generateSpinExchangeSum(d)

# reset our input
d = list(orig)

# now do all the partner swaps
doPartners(d, partners)

# once we do all the partner swaps, we can reduce the partner swaps to a smaller set
partners = generateParternSum(d)

# with the reduced maps, find the soltion to part 1
d = doSpinExchanges(orig, spins_exchanges)
doPartners(d, partners)

print ("Part 1")
printList(d)

# after we are done with part 1, spins_exchanges and partners
# holds a reduced set that maps what the output should be of each
# operation to the final result.  We can use this to iterate over
# a million transitions

d = list(orig)
for i  in range(1000000):
    d = doSpinExchanges(d, spins_exchanges)

# now generate the new map after a million transistions
spins_exchanges = generateSpinExchangeSum(d)

d = list(orig)
for i in range(1000000):
    doPartners(d, partners)

# now generate the new map after a million transistions
partners = generateParternSum(d)

# after we are done with a million transitions, spins_exchanges and partners
# holds a reduced set that maps what the output should be of a million
# operations to the final result.  We can use this to iterate over
# a thousand transitions to give us one billion

d = list(orig)
for k in range(1000):
    d = doSpinExchanges(d, spins_exchanges)
    doPartners(d, partners)

print ("Part 2")
printList(d)
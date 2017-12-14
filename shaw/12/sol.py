# start is the dictionary entry to start with
# dict is the dictionary to read/modify

# this will pull nodes out of the dictionary if they
# are connected to the start node
def findConnected(start, dict):

    i = 0
    # our list of connected nodes will contain the start node
    connected = [start]

    # while we haven't reached the end of our connected nodes list
    # note list will grow as we add to it in the loop
    while i < len(connected):
        # remove the current node from the dict so we never revisit
        pipes = dict.pop(connected[i])
        # get all the nodes the current node can talk to
        pipes = pipes.split(', ')
        # for each node
        for pipe in pipes:
            # if it is still in the dict, add it to our connected
            # list so we visit it
            if pipe in dict:
                connected.append(pipe)
        # increment our index into the connected list
        i += 1

    # return how many are in the connected list
    return len(connected)

progs = {}

with open("input") as f:
    lines = f.readlines()

# for each line, create the dictionary entry
# key is the node number (as a string)
# val is the connected nodes (as a CSV string)
for line in lines:
    vals = line.strip().split(' <-> ')
    if len(vals) == 2:
        progs[vals[0]] = vals[1]

# find all nodes connected to 0
print('part 1', findConnected('0', progs))

# now find out how many groups there are (we already found 1)
num = 1

# while stuff remains in the dictionary
while len(progs) > 0:
    # pick out a random start node
    start = next(iter(progs))
    # find all nodes connected to start node
    findConnected(start, progs)
    num += 1

print('part 2', num)
class Node(object):

    def __init__(self, line):

        # remove the commas
        line = line.replace(', ', ' ')
        # split on spaces
        vals = line.strip().split()

        # every line should have a weight
        self.weight = int(vals[1][1:-1])

        # if there are only 2 vals, there is no above list
        if len(vals) == 2:
            # total weight must be node weight
            self.total_weight = self.weight
            self.above = None

        # otherwise there is an above list, and we can't set the total weight
        else:
            self.total_weight = None
            self.above = vals[3:]

    def findBad(self, nodes):

        # if this node has a total weight set, it has already been cleared as balanced
        if self.total_weight != None:
            return False
        
        weights = []
        
        for name in self.above:
            weights.append(nodes[name].total_weight)

        # verify that all the nodes above have their total weight set
        # if not, we can't say this is bad and we'll try again next time
        if None in weights:
            return False
        
        # make sure there is only one value in the total weight list
        if len(set(weights)) == 1:
            # if so, assign this nodes total weight
            self.total_weight = self.weight
            for weight in weights:
                self.total_weight += weight
            return False

        # if there was more than one weight value in the list above, we have an imbalance
        
        # find the wrong weight
        for weight in weights:
            if weights.count(weight) == 1:
                wrong = weight
            else:
                right = weight

        # find the node with the wrong weight
        for name in self.above:
            if nodes[name].total_weight == wrong:
                print("Part 2", (nodes[name].weight - (wrong - right)))

        return True


with open("input") as f:
    lines = f.readlines()


# part 1
for i in range(len(lines)):
    # if no above list, this node can't be bottom
    if not ('->' in lines[i]):
        continue

    # get the node name
    parts = lines[i].split()
    name = parts[0]

    bottom = True
    # iterate over lines, see if this name appears in any above list
    for line in lines:
        # if no above list, name can't appear in it
        if not ('->' in line):
            continue
        (node, above) = line.split('->')
        if name in above:
            # can't be bottom if in above list
            bottom = False
            break

    # if bottom is still set, didn't find it in any above lists
    if bottom:
        print("Part 1",parts[0])
        break

# part 2
nodes = {}

# create a node based on line
for line in lines:
    vals = line.split()
    nodes[vals[0]] = Node(line)

# haven't found unbalanced node
found = False

while not found:
    # loop through every node and assign total weights if possible
    for k,v in nodes.items():
        found = v.findBad(nodes)
        if found:
            break
    # if not found, we'll iterate through the nodes again (since some weights will be updated)
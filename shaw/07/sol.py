class Node(object):

    def __init__(self, line):
        # if line indicates nodes above
        if ' -> ' in line:
            (name_weight, above) = line.split(' -> ')
            self.above = above.strip().split(', ')
        else:
            name_weight = line
            self.above = None

        # pull out name and weight
        (self.name, weight) = name_weight.split()
        self.weight = int(weight[1:-1])
        
        # if there are nodes above this one, we can't determine total_weight at the moment
        if self.above == None:
            self.total_weight = self.weight
        else:
            self.total_weight = None
          

    def link(self, nodes):
        # no nodes to link if nothing above
        if self.above is None:
            return
        
        # for every node in the above list
        for i in range(len(self.above)):
            found = False
            # find the node by name
            for node in nodes:
                if node.name == self.above[i]:
                    self.above[i] = node
                    found = True
                    break
            if not found:
                raise Exception("Couldn't link!")

    def findBad(self, nodes):

        # if this node has a total weight set, it has already been cleared as balanced
        if not self.total_weight == None:
            return False
        
        weights = []
        
        # verify that all the nodes above have their total weight set
        # if not, we can't say this is bad and we'll try again next time
        for node in self.above:
            if node.total_weight is None:
                return False
            weights.append(node.total_weight)
        
        # make sure there is only one value in the total weight list
        if len(set(weights)) == 1:
            # if so, assign this nodes total weight
            self.total_weight = 0
            for weight in weights:
                self.total_weight += weight
            self.total_weight += self.weight
            return False

        # if there was more than one weight value in the list above, we have an imbalance
        
        # find the wrong weight
        for weight in weights:
            if weights.count(weight) == 1:
                wrong = weight
            else:
                right = weight

        # find the node with the wrong weight
        for node in self.above:
            if node.total_weight == wrong:
                print("Part 2", (node.weight - (wrong - right)))

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
nodes = []

# create a node based on line
for line in lines:
    nodes.append(Node(line))

# link nodes (changes above list in node from strings to nodes)
for node in nodes:
    node.link(nodes)

# haven't found unbalanced node
found = False

while not found:
    # loop through every node and assign total weights if possible
    for node in nodes:
        found = node.findBad(nodes)
        if found:
            break
    # if not found, we'll iterate through the nodes again (since some weights will be updated)
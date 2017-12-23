NORTH = 0
EAST  = 1
SOUTH = 2
WEST  = 3

X_MOD = [0,1,0,-1]
Y_MOD = [1,0,-1,0]

num_states = 0
state_trans = []
infected_state = 0

class Node:
    def __init__(self, state, x, y):
        self.x = x
        self.y = y
        self.state = state

    def updateDir(self, dir):
        '''
        Directions can be anything from 0 to 3
        trans state tells us where to jump to based
        on the current state
        '''
        dir = (dir + state_trans[self.state]) % 4
        return dir


    def updateState(self):
        '''
        states always increment and are bounded by max # states
        '''
        self.state = ((self.state) + 1) % num_states
        return self.state

    def getNextNode(self, dir, nodes):
        '''
        based on direction an current position, find the next node
        if we've never seen it before, create it
        '''
        x = self.x + X_MOD[dir]
        y = self.y + Y_MOD[dir]

        name = 'x%dy%d' % (x,y)
        if not name in nodes:
            nodes[name] = Node(0,x,y)
        return nodes[name]

    def update(self, dir, nodes):
        '''
        determine new direction, state and node
        return
            1) if we infected the node
            2) the next node to process
            3) the direction we are facing on the next node
        '''
        dir = self.updateDir(dir)
        self.updateState()
        return (self.state == infected_state), self.getNextNode(dir, nodes), dir

def solveInfected(lines, iterations):
    '''
    cretes the base list of nodes based on input
    then determine how many infections occur
    '''
    nodes = {}

    startx = (len(lines[0].strip()) // 2)
    starty = (len(lines) // 2) * -1

    startname = 'x%dy%d' % (startx, starty)

    y = 0
    for line in lines:
        x = 0
        for c in line.strip():
            name = 'x%dy%d' % (x,y)
            if c == '#':
                nodes[name] = Node(infected_state,x,y)
            else:
                nodes[name] = Node(0,x,y)
            x += 1
        y -= 1


    node = nodes[startname]

    infect_count = 0

    dir = NORTH

    for i in range(iterations):
        infected, node, dir = node.update(dir, nodes)
        if infected:
            infect_count += 1

    print(infect_count)


with open("input") as f:
    lines = f.readlines()

# for part 1, two states
# state 0, CLEAN, causes a right turn
# state 1, INFECTED, causes a left turn (or 3 right turns)
num_states = 2
state_trans = [3, 1]
infected_state = 1

solveInfected(lines, 10000)

# for part 2, fpir states
# state 0, CLEAN, causes a right turn
# state 1, WEAKENED, does not turn
# state 2, INFECTED, causes a left turn (or 3 right turns)
# state 3, FLAGGED, causes a turn around (or 2 right turns)
num_states = 4
state_trans = [3, 0, 1, 2]
infected_state = 2

solveInfected(lines, 10000000)

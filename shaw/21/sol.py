class Grid:
    '''
    Grid class for building and maintaining states of various grids
    Can dynamically grow grids and track how much of a grid we have read
    '''
    def __init__(self):
        self.row = 0
        self.col = 0
        self.chunk = 0
        self.grid = []

    def grow(self, g, newRow):
        '''
        append tiny grid g to big grid
        if newRow is true, add to bottom
        otherwise append to last rows
        '''
        if newRow:
            for i in range(len(g)):
                self.grid.append(g[i])
        else:
            for i in range(len(g)):
                self.grid[i-len(g)] += g[i]

    def read(self):
        '''
        read out chunks of the grid
        use member vars to track state
        '''
        newRow = False
        # if we haven't figured out the chunk size for reads yet, let's do that
        # also, set flag to indicate we are reading a new row
        if self.chunk == 0:
            newRow = True
            if len(self.grid) % 2 == 0:
                self.chunk = 2
            else:
                self.chunk = 3
        # if our col tracker has us outside the grid, move down a chunk of rows
        if self.col >= len(self.grid):
            self.row += self.chunk
            self.col = 0
            newRow = True
        # if our row tracker has us outside the grid, return None
        if self.row >= len(self.grid):
            return None, None
        # return our string representation of the grid
        s = ''
        for i in range(self.chunk):
            s += self.grid[self.row+i][self.col:self.col+self.chunk]
        # increment the col tracker
        self.col += self.chunk
        return s, newRow

def rotate(s):
    '''
    helper function for rotating a grid
    '''
    if len(s) == 4:
        s = s[2] + s[0] + s[3] + s[1]
    else:
        s = s[6] + s[3] + s[0] + s[7] + s[4] + s[1] + s[8] + s[5] + s[2]
    return s

def flip(s):
    '''
    helper function for flipping a grid
    assumes 3x3 grid, since 2x2 can just be rotated
    '''
    s = s[2::-1] + s[5:2:-1] + s[8:5:-1]
    return s

# dictionary all transformation that can ocreadGrid
trans = {}

with open("input") as f:
    lines = f.readlines()

for line in lines:
    line = line.strip()
    (before, after) = line.split(' => ')

    # use a string representation of the before state (remove the /)
    # allows us to turn it into a dictionary key
    before = before.replace('/', '')

    # after state is a list of rows and their state
    after = after.split('/')

    # if the before state is 2x2
    if len(before) == 4:
        # rotate before to get all possible states that cause the after
        for i in range(4):
            trans[before] = list(after)
            before = rotate(before)

    else:
        # can't just rotate a 3x3, aslo need to mirror it
        before_flipped = flip(before)
        # rotate before to get all possible states that cause the after
        for i in range(4):
            trans[before] = list(after)
            before = rotate(before)
        # rotate the flipped before to get all possible states that cause the after
        for i in range(4):
            trans[before_flipped] = list(after)
            before_flipped = rotate(before_flipped)

def printCount(grid):
    '''counts all the # in a grid'''
    count = 0
    for s in grid.grid:
        count += s.count('#')

    print(count)

# create the start grid
readGrid = Grid()
readGrid.grow(['.#.', '..#', '###'], True)

# 18 transitions
for i in range(18):
    # print after 5 for part 1
    if i == 5:
        printCount(readGrid)

    # create a new base grid to hold transformation
    writeGrid = Grid()

    # read out the next section from the readGridrent grid
    section, newRow = readGrid.read()
    while section != None:
        # get the transformation
        result = trans[section]
        # add it to the new grid
        writeGrid.grow(result, newRow)
        # get the next section
        section, newRow = readGrid.read()
    # set our grid to read to be the new one
    readGrid = writeGrid

printCount(readGrid)
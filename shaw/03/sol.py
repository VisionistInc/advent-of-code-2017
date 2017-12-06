import math

INPUT = 265149

# get some useful constants based on input

GRID_DIM = int(math.ceil(math.sqrt(INPUT)))

# grid dimensions are always odd
if GRID_DIM % 2 == 0:
    GRID_DIM += 1

# the port will be middle of grid
PORT = GRID_DIM // 2

# this grid will be overkill for part 2, but we know it will be big enough since
# it can hold all the numbers for part 1
grid = [[0 for x in range (GRID_DIM)] for y in range(GRID_DIM)]

def part1():

    # the biggest number that can't be in the outer ring (where our input is)
    val = (GRID_DIM-2) * (GRID_DIM - 2)

    # the first number in the outer ring
    val += 1

    # each ring starts on row 1, all the way to the right
    x = GRID_DIM - 1
    y = 1

    # check if our input is along the right
    if INPUT <= (val + (GRID_DIM-2)):
        y += (INPUT - val)
    else:
        # we move to the top right corner
        val += (GRID_DIM - 2)
        y = GRID_DIM - 1
        # check INPUT on top row
        if INPUT <= (val + (GRID_DIM - 1)):
            x -= (INPUT - val)
        else:
            # we move to the bottom left corner
            val += (GRID_DIM - 1)
            x = 0
            # check if INPUT down the left side
            if INPUT <= (val + (GRID_DIM - 1)):
                y -= (INPUT - val)
            else:
                val += (GRID_DIM - 1)
                y = 0
                # has to be along bottom row
                x += (INPUT - val)

    print("Part 1 input located at: %d, %d" % (x,y))

    d = abs(x - PORT) + abs(y - PORT)
    
    print("Part 1 distance = %d" % d)


def add(x,y):
    grid[x][y] = grid[x-1][y-1] + grid[x-1][y] + grid[x-1][y+1] + grid[x][y-1] + grid[x][y+1] + grid[x+1][y-1] + grid[x+1][y] + grid[x+1][y+1]

    if grid[x][y] > INPUT:
        print("Part 2 sol = %d" % grid[x][y])
        return True

    return False

def part2():
    # assign 1 to the PORT
    grid[PORT][PORT] = 1
    x = PORT
    y = PORT

    n = 1
    m = 2

    while True:
        # we spiral out in a very defined pattern
        # from the last spot in the previous ring
        # always right 1, up n, left m, down m, right m
        # n begins as 1 and m as 2 and they increase by 2 each spiral

        # right 1
        x += 1
        if add(x,y):
            return

        # up n
        for i in range(n):
            y += 1
            if add(x,y):
                return

        # left m
        for i in range(m):
            x -= 1
            if add(x,y):
                return

        # down m
        for i in range(m):
            y -= 1
            if add(x,y):
                return

        # right m
        for i in range(m):
            x += 1
            if add(x,y):
                return

        n += 2
        m += 2

part1()
part2()



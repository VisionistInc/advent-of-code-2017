
def distance(x,y):
    # absolute for easy calc, since all relative to 0,0
    x = abs(x)
    y = abs(y)

    # if x > y, we'll be able to hit the "x-axis" and crawl along it back
    # since we can move one y with every x, y doesn't add to the distance
    # if x == y, we can take a straight diagonal to 0,0
    if x >= y:
        return x

    # now if y > x, we'll hit the y axis first, so we count all of x
    # to figure out how much more, we remove the y we were able to travel with x
    # and then divide the reamining y by 2 since we can travel 2 y in one move
    return ((y-x) / 2) + x


with open("input") as f:
    line = f.readline().strip()

x = 0
y = 0
long = 0

moves = line.split(',')

for move in moves:
    # going to use a 2d grid for the hex grid
    # we'll just have a bunch of spaces we can hit
    # if we make 'n' as y+2, 'nw' as y+1, x-1, and 'ne' as y+1, x+1
    # we can see that we can move 'nw' to get to -1,1, then 'ne' to get to 0,2
    # and this is would be the same as just moving 'n' to put us at 0,2

    if move == 'nw':
        y += 1
        x -= 1
    elif move == 'n':
        y += 2
    elif move == 'ne':
        y += 1
        x += 1
    elif move == 'se':
        y -= 1
        x += 1
    elif move == 's':
        y -= 2
    else:
        y -= 1
        x -= 1

    # save off furthest distance
    long = max(long, distance(x,y))

print('part 1', distance(x,y))
print('part 2', long)
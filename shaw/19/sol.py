with open("input") as f:
    map = f.readlines()

# want to make my life easier with bounds checking
# so i'm going to pad the map
for i in range(len(map)):
    map[i] = ' ' + map[i].strip('\n') + ' '

# add the extra row at the end
map.append(' '*len(map[0]))

row = 0
# find entry at the top
for i in map[0]:
    if i != ' ':
        col = map[0].index(i)

# our first direction will be down
row_step = 1
col_step = 0
step_count = 0
path = ''

while True:

    if map[row][col].isalpha():
        # found a letter, add it in
        path += map[row][col]

    if map[row][col] == ' ':
        # no more path, the end
        break

    if map[row][col] == '+':
        # found a change of direction
        if row_step != 0:
            # if we were heading north/south, 
            # find out if we are going east or west
            row_step = 0
            if map[row][col-1] != ' ':
                col_step = -1
            else:
                col_step = 1
        else:
            # if we were heading east/west, 
            # find out if we are going north or south
            col_step = 0
            if map[row-1][col] != ' ':
                row_step = -1
            else:
                row_step = 1

    step_count += 1

    # move to next pos
    row += row_step
    col += col_step

print("part 1", path)
print("part 2", step_count)
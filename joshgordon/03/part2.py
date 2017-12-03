#!/usr/bin/env python3

import sys
import math

if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

with open(infilename, 'r') as infile:
  buf = infile.read()

# these are entierly for pretty-printing.
directions = ("W", "S", "E", "N") #start this at 0
# what direction we're traveling. Follows the set above.
direction = 0
# our current position on the map
pos = [1, 1]
# how far we need to travel
dist = 1
# how far we've traveled in this direction
dist_state = 0

# a dict of previously seen coords and their values.
states = {(0,0):1, (1,0): 1, (1, 1): 2}

# see the details of part 1 for how this works. This is just that same algorithm, but with caching of previous states, and then
# figuring out which other previous positions we need to grab, and grabbing them and summing them.

while True:
  # before east and west, we have to increment how far we travel, but only if we just turned.
  if direction % 2 == 0 and dist_state == 0:
    dist += 1

  # list of coords to fetch out of the lookup table and sum up.
  to_sum = []

  # west
  if direction == 0:
    pos[0] -= 1
    to_sum.append((pos[0] - 1, pos[1] - 1))
    to_sum.append((pos[0]    , pos[1] - 1))
    to_sum.append((pos[0] + 1, pos[1] - 1))
    to_sum.append((pos[0] + 1, pos[1]))

  # south
  elif direction == 1:
    pos[1] -= 1
    to_sum.append((pos[0]    , pos[1] + 1))
    to_sum.append((pos[0] + 1, pos[1] - 1))
    to_sum.append((pos[0] + 1, pos[1]    ))
    to_sum.append((pos[0] + 1, pos[1] + 1))

  # east
  elif direction == 2:
    pos[0] += 1
    to_sum.append((pos[0] - 1, pos[1] + 1))
    to_sum.append((pos[0]    , pos[1] + 1))
    to_sum.append((pos[0] + 1, pos[1] + 1))
    to_sum.append((pos[0] - 1, pos[1]))

  # north
  elif direction == 3:
    pos[1] += 1
    to_sum.append((pos[0]    , pos[1] - 1))
    to_sum.append((pos[0] - 1, pos[1] - 1))
    to_sum.append((pos[0] - 1, pos[1]    ))
    to_sum.append((pos[0] - 1, pos[1] + 1))

  # increment how far we've traveled in this direction
  dist_state += 1


  # grab the sum of the coords we need to sum up.
  total = 0
  for coord in to_sum:
    print(coord)
    if coord in states:
      total += states[coord]

  states [(pos[0], pos[1])] = total

  print("position: {}, direction: {}, dist_state: {}, dist: {}, total: {}".format(
    pos,
    directions[direction],
    dist_state,
    dist,
    total
  ))

  # check for our finish condition
  if total > int(buf):
    print(total)
    break

  # check if we need to turn a corner.
  if dist == dist_state:
    dist_state = 0
    direction += 1
    direction %= 4

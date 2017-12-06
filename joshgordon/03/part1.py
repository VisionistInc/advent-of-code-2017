#!/usr/bin/env python3

import sys
import math

if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

with open(infilename, 'r') as infile:
  buf = infile.read()

# cardnial directions, just for pretty printing
directions = ("W", "S", "E", "N")
# what number we're currently on
index = 3
# what direction we're traveling
direction = 0
# what our position in the map is. Summing the absolute value of these gives us manhattan distance.
pos = [1, 1]
# how far we need to travel in the direction we're traveling
dist = 1
# how far we've traveled in the direction we're traveling
dist_state = 0


# bear with me here, but here's what's up. There's a couple of cycles that this goes through, which I'm using
# to my advantage.
#  - direction: it cycles through W, S, E, N, in that order, incrementing when we turn corners. (duh)
#  - distance: the distance traveled to go to the next turn goes in the sequence 1, 1, 2, 2, 3, 3, 4, 4, ....
# since we just need the manhattan distance, we just need the coordinate of our value, so we just need to change
# the coordinates according to the rules laid out in the bullets above.

# keep going until we find the answer.
while index != int(buf):
  # before east and west, we have to add a number to how far we travel, but only if we just turned the corner.
  if direction % 2 == 0 and dist_state == 0:
    dist += 1

  # west - x coord gets decremented
  if direction == 0:
    pos[0] -= 1

  # south - y coord gets decremented
  elif direction == 1:
    pos[1] -= 1

  # east - x coord gets incremented
  elif direction == 2:
    pos[0] += 1

  # north - y coord gets incremented
  elif direction == 3:
    pos[1] += 1

  # travel closer to the corner
  dist_state += 1
  # keep the number moving
  index += 1

  # print our state
  print("position: {}, direction: {}, dist_state: {}, dist: {}, index: {}".format(
    pos,
    directions[direction],
    dist_state,
    dist,
    index
  ))


  # check if we need to turn a corner.
  if dist == dist_state:
    dist_state = 0
    direction += 1
    direction %= 4


print(pos)
print(sum([abs(pos[0]), abs(pos[1])]))

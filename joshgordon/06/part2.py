#!/usr/bin/env python3

import sys
import operator


if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

previous_states = []
cycles = 0

with open(infilename, 'r') as infile:
  current_state = [int(i) for i in infile.read().strip().split('\t')]

# while we haven't finished, keep iterating
while tuple(current_state) not in previous_states:
  cycles += 1

  # stash a tuple of our previous states so we can check back later.
  previous_states.append(tuple(current_state))
  print(current_state)

  # find the index/value with the maximum value.
  index, value = max(enumerate(current_state), key=operator.itemgetter(1))
  # set it to 0
  current_state[index] = 0

  # go through and do the re-distribution
  for i in range(value):
    idx = (index + i + 1) % len(current_state)
    current_state[idx] += 1

# figure out where we've seen that value before...
previously_seen = previous_states.index(tuple(current_state))
# print out our answer.
print(cycles - previously_seen)

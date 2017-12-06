#!/usr/bin/env python3

import sys


if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

instructions = []
ip = 0
steps = 0
with open(infilename, 'r') as infile:
  for line in infile:
    instructions.append(int(line.strip()))

while ip < len(instructions):
  steps += 1
  offset = instructions[ip]
  if offset >= 3:
    instructions[ip] -= 1
  else:
    instructions[ip] += 1
  ip += offset

print steps

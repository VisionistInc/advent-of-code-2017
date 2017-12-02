#!/usr/bin/env python3

import sys

def is_divisible(rest):
  divisibles = []
  num = rest[0]
  rest = rest[1:]
  for i in rest:
    if num % i == 0 or i % num == 0:
      divisibles.append(i)
  return divisibles

if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

with open(infilename, 'r') as infile:
  buf= [line.strip().split() for line in infile]

max_col = max([len(x) for x in buf])
print max_col



row_checksums = []
for line in buf:
  line = [int(i) for i in line]
  for i in range(len(line)):
    divisibles = is_divisible(line[i:])
    if len(divisibles) > 0:
      row_checksums.append(max(divisibles[0], line[i])/ min(divisibles[0], line[i]))
print sum(row_checksums)

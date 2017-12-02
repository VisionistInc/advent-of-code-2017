#!/usr/bin/env python3

import sys

# find what numbers are divisible, between the first of the array and
# the rest of the array.
def is_divisible(rest):
  divisibles = []
  num = rest[0]
  rest = rest[1:]
  for i in rest:
    if num % i == 0 or i % num == 0:
      return i
  return None

if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

with open(infilename, 'r') as infile:
  buf= [line.strip().split() for line in infile]

max_col = max([len(x) for x in buf])
print(max_col)



row_checksums = []
for line in buf:
  line = [int(i) for i in line]
  for i in range(len(line)):
    divisibles = is_divisible(line[i:])
    if divisibles is not None:
      row_checksums.append(max(divisibles, line[i])/ min(divisibles, line[i]))
print(sum(row_checksums))

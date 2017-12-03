#!/usr/bin/env python3

import sys


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
  maximum = 0
  minimum = 1000000000
  for i in range(len(line)):
    maximum = max(maximum, int(line[i]))
    minimum = min(minimum, int(line[i]))
  row_checksums.append(int(maximum) - int(minimum))
  print(maximum, minimum)
print(sum(row_checksums))

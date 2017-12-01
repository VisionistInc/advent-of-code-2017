#!/usr/bin/env python3

import sys

print("hello world")

if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

with open(infilename, 'r') as infile:
  buf= infile.read().strip()

sum = 0

for ii in range(len(buf)):
  jj = (ii+1) % len(buf)
  print("ii {:d}, jj {:d}".format(ii, jj))
  if buf[ii] == buf[jj]:
    sum += int(buf[ii])


print("Sum is {:d}".format(sum))

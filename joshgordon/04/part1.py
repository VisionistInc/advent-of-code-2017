#!/usr/bin/env python3

import sys


if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

passwords = []

with open(infilename, 'r') as infile:
  for line in infile:
    passwords.append(line.strip().split(' '))

count = 0
for password in passwords:
  if len(password) == len(set(password)):
    count += 1

print(count)


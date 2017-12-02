#! /usr/bin/env python3
import os
import datetime
import requests
session_cookie = os.environ.get('aoc_session', '')
day = datetime.datetime.now().day

os.system('mkdir {:02d}'.format(day))

response = requests.get('https://adventofcode.com/2017/day/{}/input'.format(day), cookies={'session': session_cookie})
if response.status_code == 200:
  with open('{:02d}/input.txt'.format(day), 'w') as f:
    f.write(response.text)
else:
  print("Failed to fetch input with status code {} and response text {}".format(response.status_code, response.text))

template="""#!/usr/bin/env python3

import sys


if len(sys.argv) > 1:
  infilename = sys.argv[1]
else:
  infilename = 'input.txt'

with open(infilename, 'r') as infile:
  buf = infile.read()
  print buf
"""

with open('{:02d}/part1.py'.format(day), 'w') as f:
  f.write(template)

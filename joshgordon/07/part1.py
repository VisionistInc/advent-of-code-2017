#!/usr/bin/env python3
# Yes, I am aware that I just switched from 2 spaces to 4 spaces. I installed linux on my desktop and this
# is the default formatting from vim, and also I remembered that it's the preferred tab-width for python.
# I am truly and deeply sorry if this has offended you. If you would like to seek financial compensation
# for this tradgedy, please email me at the address on my github profile, and I'll send you like...
# 5 cents, followed by seeking financial compensation for you seeking financial compensation from me...

import re
import sys


# get some regexes compiled:
node_re = re.compile(r'(\w+) \((\d+)\)')
relationship_re = re.compile(r'(\w+) \((\d+)\) -> (.+)')

if len(sys.argv) > 1:
    infilename = sys.argv[1]
else:
    infilename = 'input.txt'


parent_nodes = set()
child_nodes = set()

with open(infilename, 'r') as infile:
    lines = [line.strip() for line in infile]

for line in lines:
    if '->' not in line:
        match = node_re.match(line)
        child_nodes.add(match.group(1))

    elif '->' in line:
        match = relationship_re.match(line.strip())
        parent_nodes.add(match.group(1))

        children = [n.strip() for n in match.group(3).split(',')]
        for child in children:
            child_nodes.add(child)



print(parent_nodes - child_nodes)

#!/usr/bin/env python3
# Yes, I am aware that I just switched from 2 spaces to 4 spaces. I installed linux on my desktop and this
# is the default formatting from vim, and also I remembered that it's the preferred tab-width for python.
# I am truly and deeply sorry if this has offended you. If you would like to seek financial compensation
# for this tradgedy, please email me at the address on my github profile, and I'll send you like...
# 5 cents, followed by seeking financial compensation for you seeking financial compensation from me...

import re
import sys


class Node:
    children = None
    name = ""
    weight = 0
    def __init__(self, name, weight, children=None):
        self.name = name
        self.weight = int(weight)
        if children is not None:
            self.children = children

    def get_weight(self):
        if self.children is not None:
            return self.weight + sum([child.get_weight() for child in self.children])
        else:
            return self.weight

    def get_children_with_weight(self):
        if self.children is not None:
            return [(child.name, child.get_weight()) for child in self.children]
        else:
            return []

    def is_balanced(self):
        if self.children is not None:
            weights = {}
            for child in self.children:
                child_weight = child.get_weight()
                if child_weight in weights:
                    weights[child_weight].append(child)
                else:
                    weights[child_weight] = [child]

            # all of my children are balanced, so I could be the problematic one.
            if len(weights) == 1:
                return True
            else:
                return False


    def find_unbalanced_tower(self):
        print("FInding unbalanced tower for {}".format(self.name))
        if self.children is None:
            print("{} has no children, returning None".format(self.name))
            return None

        elif self.is_balanced():
            print("{} is balanced, returning None".format(self.name))
            return None
        # my children are not balanced, so keep digging until we get there.
        else:
            print("{} is unbalanced and has children, or something like that.".format(self.name))
            my_fault = True
            for child in self.children:
                if not child.is_balanced():
                    print("My child, {} is not balanced.".format(child.name))
                    my_fault = False
                    blame_child = child.find_unbalanced_tower()
                    if blame_child is None:
                        return None
                    else:
                        print("Found problem with {} whose total weight is {}. All his children are balanced.".format(blame_child.name, blame_child.get_weight()))
                        print("Why can't he be more like my other children, whose weights are: ".format(", ".join([str(child.get_weight()) for child in self.children])))
                        return None

            # all my children are balanced within themselves, one of them themselves must be doing it.
            if my_fault:
                print("One of these is not like the other: {}".format(", ".join([child.name + ": " + str(child.get_weight()) for child in self.children])))
                for child in self.children:
                    print(child)
                    


    def __repr__(self):
        if self.children is None:
            return "{} ({})".format(self.name, self.weight)
        else:
            return "{} ({}) -> {}".format(self.name, self.weight, ", ".join(["{} ({}, {} (total))".format(child.name, child.weight, child.get_weight()) for child in self.children]))


# get some regexes compiled:
node_re = re.compile(r'(\w+) \((\d+)\)')
relationship_re = re.compile(r'(\w+) \((\d+)\) -> (.+)')

if len(sys.argv) > 1:
    infilename = sys.argv[1]
else:
    infilename = 'input.txt'


parent_nodes = set()
child_nodes = set()
nodes = {}

with open(infilename, 'r') as infile:
    lines = [line.strip() for line in infile]

for line in lines:
    match = node_re.match(line)
    if '->' not in line:
        child_nodes.add(match.group(1))
    node = Node(match.group(1), match.group(2))
    nodes[node.name] = node

        

for line in lines:
    if '->' in line:
        match = relationship_re.match(line.strip())
        node_name = match.group(1)
        parent_nodes.add(match.group(1))

        children = [n.strip() for n in match.group(3).split(',')]
        children_nodes = []
        for child in children:
            child_nodes.add(child)
            children_nodes.append(nodes[child])

        if node_name in nodes:
            nodes[node_name].children = children_nodes
        else:
            node = Node(match.group(1), match.group(2), children_nodes)
            nodes[node.name] = node

root = (parent_nodes - child_nodes).pop()
print(nodes[root].get_weight())

print(nodes[root].find_unbalanced_tower())

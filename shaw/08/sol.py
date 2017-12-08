with open("input") as f:
    lines = f.readlines()

d = {}

# build dictionary with 0 vals
for line in lines:
	vals = line.split()
	d[vals[0]] = 0
	d[vals[4]] = 0

max_num = None

for line in lines:
	(dst, inc, delta, word, reg, cond, val) = line.strip().split()

	delta = int(delta)
	val = int(val)

	doIt = False

	if cond == '==' and d[reg] == val:
		doIt = True
	if cond == '!=' and d[reg] != val:
		doIt = True
	if cond == '>=' and d[reg] >= val:
		doIt = True
	if cond == '<=' and d[reg] <= val:
		doIt = True
	if cond == '>' and d[reg] > val:
		doIt = True
	if cond == '<' and d[reg] < val:
		doIt = True

	if doIt:
		if inc == 'inc':
			d[dst] = d[dst] + delta
		else:
			d[dst] = d[dst] - delta

	# for part 2, look for largest ever seen
	if max_num is None:
		max_num = d[dst]
	else:
		if d[dst] > max_num:
			max_num = d[dst]

print('part2', max_num)

max_num = None

for key in d:
	if max_num is None:
		max_num = d[key]
	else:
		if d[key] > max_num:
			max_num = d[key]

print('part1', max_num)
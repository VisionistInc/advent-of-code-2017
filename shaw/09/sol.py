with open("input") as f:
    line = f.readline()

level = 1
score = 0
trash = False
skip = False

trash_count = 0

for c in line:
	if trash:
		if skip:
			skip = False
			continue
		if c == '!':
			skip = True
			continue
		if c == '>':
			trash = False
			continue
		trash_count += 1
		continue

	if c == '<':
		trash = True
		continue
	if c == '{':
		score += level
		level += 1
		continue
	if c == '}':
		level -= 1

print("Part 1", score)
print("Part 2", trash_count)
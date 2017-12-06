

with open("input") as f:
    lines = f.readlines()

sol1 = 0
sol2 = 0

for line in lines:
    sol1_flag = True
    sol2_flag = True
    sol1_words = line.split()
    sol2_words = line.split()

    if len(sol1_words) == 0:
        continue

    # part 1, just look for the same word in remainder of list
    for x in range(0, len(sol1_words)):
        if sol1_words[x] in sol1_words[x+1:]:
            sol1_flag = False

    # part 2, we alphabetize all the letters in the words
    # so any anagrams turn into the same word
    for x in range(0, len(sol2_words)):
        sol2_words[x] = ''.join(sorted(sol2_words[x]))

    # now we look to see if the same word in remainder of list
    for x in range(0, len(sol2_words)):
        if sol2_words[x] in sol2_words[x+1:]:
            sol2_flag = False

    if sol1_flag:
        sol1 += 1
    if sol2_flag:
        sol2 += 1

print(sol1, sol2)

# i don't feel like parsing a bunch of text, so i'll do this as input

NUM_STEPS = 12586542

states = {}
states['A'] = [[1,1,'B'],[0,-1,'B']]
states['B'] = [[0,1,'C'],[1,-1,'B']]
states['C'] = [[1,1,'D'],[0,-1,'A']]
states['D'] = [[1,-1,'E'],[1,-1,'F']]
states['E'] = [[1,-1,'A'],[0,-1,'D']]
states['F'] = [[1,1,'A'],[1,-1,'E']]

# start the tape off with one 0 entry at position 0 in state A
tape = [0]
pos = 0
state = 'A'

for i in range(NUM_STEPS):
    val = tape[pos]
    next_pos = pos + states[state][val][1]
    next_state = states[state][val][2]
    tape[pos] = states[state][val][0]
    pos = next_pos
    state = next_state

    # if we've gone too far left, add one to the start of the list
    if pos == -1:
        pos = 0
        tape.insert(0,0)

    # if we've gone too far right, add one to the end of the list
    elif pos == len(tape):
        tape.append(0)

print(tape.count(1))
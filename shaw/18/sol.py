with open("input") as f:
    lines = f.readlines()

regs = {}
code = []

# create registers and code
for line in lines:
    inst = line.strip().split()
    # for each param, try and change to int
    # if it fails, it's a register name
    for i in range(1, len(inst)):
        try:
            inst[i] = int(inst[i])
        except:
            regs[inst[i]] = 0
    code.append(list(inst))

# set program counter and sound value
pc = 0
sound = 0

# execute
while True:
    # pull out the values
    try:
        (inst, r1, r2) = code[pc]
    except:
        (inst, r1) = code[pc]

    if r1 in regs:
        r1_val = regs[r1]
    else:
        r1_val = r1
    if r2 in regs:
        r2_val = regs[r2]
    else:
        r2_val = r2

    if inst == 'snd':
        sound = r1_val
        pc += 1
    elif inst == 'rcv':
        regs[r1] = sound
        print("part 1", sound)
        break
        pc += 1
    elif inst == 'set':
        regs[r1] = r2_val
        pc += 1
    elif inst == 'add':
        regs[r1] += r2_val
        pc += 1
    elif inst == 'mul':
        regs[r1] *= r2_val
        pc += 1
    elif inst == 'mod':
        regs[r1] %= r2_val
        pc += 1
    else:
        if r1_val > 0:
            pc += r2_val
        else:
            pc += 1

# zero out regs
for key in regs:
    regs[key] = 0

# make unique regs for each program
p0_regs = dict(regs)
p1_regs = dict(regs)

# set initial state
p1_regs['p'] = 1

# create queues
q_to_p0 = []
q_to_p1 = []

# create send counters
send_to_p0 = 0
send_to_p1 = 0

# init program pcs
p0_pc = 0
p1_pc = 0

# execute
while True:
    # run p0 until it can't run any more (nothin to recv)
    while True:
        try:
            (inst, r1, r2) = code[p0_pc]
        except:
            (inst, r1) = code[p0_pc]

        if r1 in p0_regs:
            r1_val = p0_regs[r1]
        else:
            r1_val = r1
        if r2 in p0_regs:
            r2_val = p0_regs[r2]
        else:
            r2_val = r2

        if inst == 'snd':
            q_to_p1.append(r1_val)
            send_to_p1 += 1
            p0_pc += 1
        elif inst == 'rcv':
            if len(q_to_p0) > 0:
                p0_regs[r1] = q_to_p0.pop(0)
                p0_pc += 1
            else:
                # nothing to recv, yield
                break
        elif inst == 'set':
            p0_regs[r1] = r2_val
            p0_pc += 1
        elif inst == 'add':
            p0_regs[r1] += r2_val
            p0_pc += 1
        elif inst == 'mul':
            p0_regs[r1] *= r2_val
            p0_pc += 1
        elif inst == 'mod':
            p0_regs[r1] %= r2_val
            p0_pc += 1
        else:
            if r1_val > 0:
                p0_pc += r2_val
            else:
                p0_pc += 1

    # run p1 until it can't run any more
    while True:
        try:
            (inst, r1, r2) = code[p1_pc]
        except:
            (inst, r1) = code[p1_pc]

        if r1 in p1_regs:
            r1_val = p1_regs[r1]
        else:
            r1_val = r1
        if r2 in p1_regs:
            r2_val = p1_regs[r2]
        else:
            r2_val = r2

        if inst == 'snd':
            q_to_p0.append(r1_val)
            send_to_p0 += 1
            p1_pc += 1
        elif inst == 'rcv':
            if len(q_to_p1) > 0:
                p1_regs[r1] = q_to_p1.pop(0)
                p1_pc += 1
            else:
                # nothing to recv, yield
                break
        elif inst == 'set':
            p1_regs[r1] = r2_val
            p1_pc += 1
        elif inst == 'add':
            p1_regs[r1] += r2_val
            p1_pc += 1
        elif inst == 'mul':
            p1_regs[r1] *= r2_val
            p1_pc += 1
        elif inst == 'mod':
            p1_regs[r1] %= r2_val
            p1_pc += 1
        else:
            if r1_val > 0:
                p1_pc += r2_val
            else:
                p1_pc += 1


    # if both queues are empty, nothing to recv, deadlock
    if len(q_to_p1) == 0 and len(q_to_p0) == 0:
        break

print("part 2", send_to_p0)
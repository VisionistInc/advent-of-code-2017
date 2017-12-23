'''
There's a lot of cruft in here
I just took Day 18, added the extra instrucions
and counted calls to mul
'''

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
mul_count = 0

# execute
while pc < len(code):
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
        pc += 1
    elif inst == 'set':
        regs[r1] = r2_val
        pc += 1
    elif inst == 'add':
        regs[r1] += r2_val
        pc += 1
    elif inst == 'sub':
        regs[r1] -= r2_val
        pc += 1
    elif inst == 'mul':
        mul_count += 1
        regs[r1] *= r2_val
        pc += 1
    elif inst == 'mod':
        regs[r1] %= r2_val
        pc += 1
    elif inst == 'jnz':
        if r1_val != 0:
            pc += r2_val
        else:
            pc += 1
    else:
        if r1_val > 0:
            pc += r2_val
        else:
            pc += 1

print(mul_count)

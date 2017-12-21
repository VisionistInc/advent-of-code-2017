def sum(l):
    '''Manhatten distance from 0 for [x,y,z]'''
    return abs(l[0]) + abs(l[1]) + abs(l[2])

class Particle:

    def __init__(self, id, s):
        '''
        ints class members
        id is the id of the particle
        pos is a list of [x,y,z] position
        vel is a list of [x,y,z] velocity
        acc is a list of [x,y,z] acceleration
        dis is the current distance from 0
        '''
        self.id = id
        
        s = s.strip().split(', ')
        
        pos = s[0][3:-1].split(',')
        self.pos = list(map(int,pos))
        
        vel = s[1][3:-1].split(',')
        self.vel = list(map(int,vel))
        
        acc = s[2][3:-1].split(',')
        self.acc = list(map(int,acc))
        
        self.dis = sum(self.pos)

    def step(self):
        '''
        adjusts the velocity then position of the class
        updates the distance as well
        '''
        for i in range(3):
            self.vel[i] += self.acc[i]
            self.pos[i] += self.vel[i]
            
        self.dis = sum(self.pos)
    
    def pos_match(self, part):
        '''
        determines if the [x,y,z] positions match for two particles
        '''
        if part.pos[0] == self.pos[0]:
            if part.pos[1] == self.pos[1]:
                if part.pos[2] == self.pos[2]:
                    return True
        return False

def removeCrashes(l):
    ''' 
    for a list of particles, find any that match in position and remove them
    '''
    to_pop = []
    for part in l:
        for part2 in l[l.index(part)+1:]:
            if part.pos_match(part2):
                to_pop.append(l.index(part))
                to_pop.append(l.index(part2))
    # remove duplicates
    to_pop = list(set(to_pop))
    # reverse the order so that later particles are first
    # this way we don't mess with index when we pop
    to_pop = reversed(sorted(to_pop))

    # remove all dupes
    for i in to_pop:
        l.pop(i)

def diverging(l):
    '''
    determines if a list of particles are all diverging in direction
    '''
    x = []
    y = []
    z = []
    # for every particle, add their acceleration and their position
    # for each direction
    for part in l:
        x.append([part.acc[0], part.pos[0]])
        y.append([part.acc[1], part.pos[1]])
        z.append([part.acc[2], part.pos[2]])
    # now sort based on the acceleration first
    # then the position along the axis second
    x = sorted(x, key=lambda j: (j[0], j[1]))
    y = sorted(y, key=lambda j: (j[0], j[1]))
    z = sorted(z, key=lambda j: (j[0], j[1]))

    # now we have everything in acceleration order
    # if the positions are also in order
    # then it will be impossible for a lesser position to overtake
    # a larger one, since the larger one should be accelerating at
    # or faster than the lesser one

    # if we find any positions out of order, we haven't
    # reached a steady state yet, so particles can still collide
    pos = x[0][1]
    for l in x[1:]:
        if l[1] < pos:
            return False
        pos = l[1]
    
    pos = y[0][1]
    for l in y[1:]:
        if l[1] < pos:
            return False
        pos = l[1]
    
    pos = z[0][1]
    for l in z[1:]:
        if l[1] < pos:
            return False
        pos = l[1]
    return True

with open("input") as f:
    lines = f.readlines()

# create list of particles based on lines read in
particles = []
id = 0
for line in lines:
    particles.append(Particle(id, line))
    id += 1

# while some particles are still approaching 0
closing_in = True
while closing_in:
    closing_in = False
    for part in particles:
        old_d = part.dis
        part.step()
        # if the new distance is closer than the old one
        # we still have particles approaching 0
        if part.dis < old_d:
            closing_in = True

# now that we are no longer closing in
# we can find the one that will reamin closest to 0
slowest = sum(particles[0].acc)
closest = particles[0].dis
particle = 0

# the goal is to find the one with the least acceleration
# if accelerations are the same, tie goes to the once
# currently closest to 0
for part in particles:
    if sum(part.acc) < slowest:
        slowest = sum(part.acc)
        closest = part.dis
        particle = part.id
    elif sum(part.acc) == slowest and part.dis < closest:
        closest = part.dis
        particle = part.id

print("part 1", particle)

# recreate the initial state
particles = []
id = 0
for line in lines:
    particles.append(Particle(id, line))
    id += 1

# remove any particles that are crashing in the initial state
removeCrashes(particles)

# while there is still potential for crashing
while not diverging(particles):
    for part in particles:
        part.step()
    
    # once everything moved, check for new collisions
    removeCrashes(particles)

print("part 2", len(particles))

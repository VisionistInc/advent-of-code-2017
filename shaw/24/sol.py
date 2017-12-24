class Component:
    def __init__(self, s):
        self.used = False
        self.nums = line.strip().split('/')
        self.nums = list(map(int, self.nums))
        self.total = self.nums[0] + self.nums[1]

    def canUse(self, num):
        if self.used:
            return None
        if num in self.nums:
            self.used = True
            next_num = self.nums.index(num) - 1
            return self.nums[next_num]
        
        return None

    def sum(self):
        if self.used:
            return self.total
        return 0

with open("input") as f:
    lines = f.readlines()

components = []
for line in lines:
    components.append(Component(line))

strongest_overall = 0
longest = 0
strongest_long = 0

def find(num, depth):
    '''
    find any unused components that have a mate for num
    call recursively on the partner num of the
    component found
    '''
    global strongest_overall
    global longest
    global strongest_long

    for i in range(len(components)):
        next_num = components[i].canUse(num)
        if next_num == None:
            continue
        find(next_num, depth+1)
        # reset used flag
        components[i].used = False
    
    strength = 0
    for i in range(len(components)):
        strength += components[i].sum()

    strongest_overall = max(strongest_overall, strength)
    
    if depth >= longest:
        longest = depth
        strongest_long = max(strongest_long, strength)

find(0,0)

print(strongest_overall)
print(longest, strongest_long)
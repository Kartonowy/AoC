import math
class Pair:
    left = ""
    right = ""
    def __init__(self, left, right):
        self.left = left
        self.right = right

map_dict = {}

f = open('input.txt')
turns = f.readline()

print("turns = " + turns)

instructions = f.readlines()
chars = "=()\n,"

for instruction in instructions:
    for character in chars:
        instruction = instruction.replace(character, "").replace("  ", " ")
    if instruction == '':
        continue
    chains = instruction.split(" ")
    print(instruction)
    map_dict[chains[0]] = Pair(chains[1], chains[2])


current = "AAA"
pointer = 0
counter = 0
while current != "ZZZ":
    if turns[pointer] == "R":
        # print("is " + current + " moving to " + map_dict[current].right)
        current = map_dict[current].right
        counter += 1
    if turns[pointer] == "L":
        # print("is " + current + " moving to " + map_dict[current].left)
        current = map_dict[current].left
        counter += 1

    pointer += 1
    if pointer >= len(turns):
        pointer = 0


print("Part 1 = " + str(counter))

started = []
def check_end(nodes):
    all_true = True
    for node in nodes:
        if node[2] != 'Z':
            all_true = False
    return all_true
            
used = []

for key in map_dict:
    if key[2] == "A":
        used.append(key)

print(used)

counter = 1
results = []
for code in used:
    sec_pointer = 0
    internal = 0
    while code[2] != "Z":
        if turns[sec_pointer] == "R":
        # print("is " + current + " moving to " + map_dict[current].right)
            code = map_dict[code].right
            internal += 1
        if turns[sec_pointer] == "L":
        # print("is " + current + " moving to " + map_dict[current].left)
            code = map_dict[code].left
            internal += 1

        sec_pointer += 1
        if sec_pointer >= len(turns):
            sec_pointer = 0

    results.append(internal)

print("Part 2 = " + str(math.lcm(*results)))
operations = []

def get_masks(bitstr):
    pos, neg = "", ""
    for b in bitstr:
        if b == "0":
            neg += "1"
            pos += "0"
        elif b == "1":
            pos += "1"
            neg += "0"
        else:
            pos += "0"
            neg += "0"
    return int(pos, 2), int(neg,2)

def get_gmask(g):
    if g == "1":
        return 1, 1
    if g == "2":
        return 2, 2
    if g == "3":
        return 3, 0
    if g == "X":
        return "X", "X"
    raise Exception("Bad g: {}".format(g))

with open("pla.txt") as fin:
    for line in fin:
        ni, g, t, name = line.strip().split()
        operations.append((ni, g, t, name))

seen = set()
for ni, g, t, name in operations:
    if (ni, g, t) in seen:
        continue

    pos, neg = get_masks(ni)
    gmask, gval = get_gmask(g)
    if g == "X" and t == "X":
        print("\t\t" + "if self.opcode & 0x{0:02x} == 0x{0:02x} && !self.opcode & 0x{1:02x} == 0x{1:02x} {{".format(pos, neg))
        print("\t\t" + "\t// {}".format(name))
        print("\t\t" + "\tunimplemented!();")
        print("\t\t" + "}")
    elif g == "X":
        print("\t\t" + "if self.opcode & 0x{0:02x} == 0x{0:02x} && !self.opcode & 0x{1:02x} == 0x{1:02x} && self.time == {2} {{".format(pos, neg, t))
        print("\t\t" + "\t// {}".format(name))
        print("\t\t" + "\tunimplemented!();")
        print("\t\t" + "}")
    elif t == "X":
        print("\t\t" + "if self.opcode & 0x{0:02x} == 0x{0:02x} && !self.opcode & 0x{1:02x} == 0x{1:02x} && g & {2} == {3} {{".format(pos, neg, gmask, gval))
        print("\t\t" + "\t// {}".format(name))
        print("\t\t" + "\tunimplemented!();")
        print("\t\t" + "}")
    else:
        print("\t\t" + "if self.opcode & 0x{0:02x} == 0x{0:02x} && !self.opcode & 0x{1:02x} == 0x{1:02x} && g & {2} == {3} && self.time == {4} {{".format(pos, neg, gmask, gval, t))
        print("\t\t" + "\t// {}".format(name))
        print("\t\t" + "\tunimplemented!();")
        print("\t\t" + "}")

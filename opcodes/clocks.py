import sys
import os
import re

if len(sys.argv) < 2:
    print("usage: {0} <value>".format(sys.argv[0]))
    sys.exit()

opc = eval(sys.argv[1])
b0 = (opc & 1) != 0
b1 = (opc & 2) != 0
gmatch = (False, b0, b1, not b0 and not b1)
bin = "{:08b}".format(opc)

input = []
with open("pla.txt") as fin:
    for s in fin:
        s = s.strip()
        if s.startswith("#"):
            continue
        input.append(s.split())

for i in range(6):
    print("T={}".format(i))
    for ni, g, t, name in input:
        instr_match = re.match(ni.replace("X", "."), bin)
        time_match = (t == "X" or int(t) == i)
        g_match = (g == "X" or gmatch[int(g)])
        if instr_match and time_match and g_match:
            print("{} {} {} {}".format(ni, g, t, name))
    print("")

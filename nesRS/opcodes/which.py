import sys
import os
import re

if len(sys.argv) < 5:
    print("usage: {0} pos_mask neg_mask time gval".format(sys.argv[0]))

pmask = int(sys.argv[1], base=16)
nmask = int(sys.argv[2], base=16)
time = sys.argv[3]
gval = int(sys.argv[4])


with open("ops_table.txt") as fin:
    for s in fin:
        s = s.strip()
        match = re.match("([0-9A-Fa-f]{2})\s+([A-Za-z*]+)\s+(?:([aimpxz]+)\s+)?(\d+)?", s)
        opcode = int(match.group(1), base = 16)
        opname = match.group(2)
        address_mode = match.group(3)
        cycles = match.group(4)

        if cycles is not None and time != "X":
            c = int(cycles)
            t = int(time)
            if cycles < time:
                continue

        b0 = (opcode & 1) != 0
        b1 = (opcode & 2) != 0
        gmatch = (False, b0, b1, not b0 and not b1)
        if not gmatch[gval]:
            continue

        if (opcode & pmask == pmask) and (~opcode & nmask == nmask):
            print(hex(opcode), opname)

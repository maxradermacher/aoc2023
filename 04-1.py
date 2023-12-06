import sys

def parse_numbers(ns):
    return [int(n) for n in ns.strip().replace("  ", " ").split(" ")]

r = 0
for line in sys.stdin:
    line = line[line.index(":")+1:]
    (winners, candidates) = map(parse_numbers, line.split("|"))
    matches = set(winners) & set(candidates)
    if len(matches) > 0:
        r += 2**(len(matches) - 1)
print(r)

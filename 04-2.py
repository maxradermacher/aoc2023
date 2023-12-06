import sys

def parse_numbers(ns):
    return [int(n) for n in ns.strip().replace("  ", " ").split(" ")]

lines = list(sys.stdin)
count = [1 for _ in range(len(lines))]
for (i, line) in enumerate(lines):
    line = line[line.index(":")+1:]
    (winners, candidates) = map(parse_numbers, line.split("|"))
    matches = set(winners) & set(candidates)
    for k in range(i+1, i+1+len(matches)):
        count[k] += count[i]
print(sum(count))
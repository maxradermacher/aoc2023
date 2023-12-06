import operator
import sys

lines = sys.stdin.readlines()
seeds = list(map(int, lines[0][len("seeds:"):].strip().split(" ")))
lines = lines[2:]

tables = []
for line in lines:
    line = line.strip()
    if ":" in line:
        tables.append([])
    elif len(line) > 0:
        tables[-1].append(tuple(map(int, line.split(" "))))

tables = [sorted(table, key=operator.itemgetter(1)) for table in tables]

r = []
for seed in seeds:
    v = seed
    for table in tables:
        for (dst, src, n) in table:
            if v >= src+n:
                continue
            if v >= src:
                v = dst + (v-src)
            break
    r.append(v)
print(min(r))
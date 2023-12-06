import operator
import sys

lines = sys.stdin.readlines()
seeds = list(map(int, lines[0][len("seeds:"):].strip().split(" ")))
seeds = [(seeds[n], seeds[n+1]) for n in range(0, len(seeds), 2)]
lines = lines[2:]

tables = []
for line in lines:
    line = line.strip()
    if ":" in line:
        tables.append([])
    elif len(line) > 0:
        tables[-1].append(tuple(map(int, line.split(" "))))

tables = [sorted(table, key=operator.itemgetter(1)) for table in tables]

this_seeds = seeds
for table in tables:
    next_seeds = []
    for (sv, sn) in this_seeds:
        def add(dv, dn, sv, sn):
            dn = min(dn, sn)
            next_seeds.append((dv, dn))
            return (sv+dn, sn-dn)

        for (dst, src, n) in table:
            if sn == 0:
                break
            if sv < src:
                sv, sn = add(sv, src-sv, sv, sn)
            if sn == 0:
                break
            if sv < src+n:
                sv, sn = add(dst + (sv-src), src+n-sv, sv, sn)
        if sn > 0:
            sv, sn = add(sv, sn, sv, sn)
    this_seeds = next_seeds
print(min(seed for seed, _ in this_seeds))

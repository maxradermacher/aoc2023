import sys

def parse_numbers(ns):
    return [int(n) for n in ns.strip().split(" ") if len(n) > 0]

times = parse_numbers(sys.stdin.readline()[len("Times:"):])
dists = parse_numbers(sys.stdin.readline()[len("Distances:"):])

r = 1
for (time, record) in zip(times, dists):
    ns = []
    for n in range(time):
        me = n * (time - n)
        if me > record:
            ns.append(n)
    r *= ns[-1] - ns[0] + 1
print(r)

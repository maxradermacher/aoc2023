import sys

def parse_numbers(ns):
    return [n for n in ns.strip().split(" ") if len(n) > 0]

time = int("".join(parse_numbers(sys.stdin.readline()[len("Times:"):])))
dist = int("".join(parse_numbers(sys.stdin.readline()[len("Distances:"):])))

lo = 0
hi = time//2
while lo < hi:
    mid = (lo + hi) // 2
    me = mid * (time - mid)
    if me <= dist:
        lo = mid + 1
    else:
        hi = mid
print(time - lo - lo + 1)

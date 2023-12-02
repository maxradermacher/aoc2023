import sys

r = 0
for l in sys.stdin:
    ds = list(filter(str.isdigit, l))
    d1 = ds[0]
    d2 = ds[-1]
    r += int(d1 + d2)
print(r)

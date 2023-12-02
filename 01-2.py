import sys

T = {
    "one": 1,
    "two": 2,
    "three": 3,
    "four": 4,
    "five": 5,
    "six": 6,
    "seven": 7,
    "eight": 8,
    "nine": 9,
}

def digs(l):
    r = []
    while len(l) > 0:
        if l[0].isdigit():
            r.append(l[0])
        for c in (3, 4, 5):
            v = T.get(l[:c])
            if v is not None:
                r.append(str(v))
        l = l[1:]
    return r

r = 0
for l in sys.stdin:
    ds = digs(l)
    d1 = ds[0]
    d2 = ds[-1]
    r += int(d1 + d2)
print(r)

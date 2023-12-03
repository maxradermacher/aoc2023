import sys

r = 0
numbers = []
symbols = []
numbers.append([])
for line in sys.stdin:
    numbers.append([])
    symbols.append([])
    for (col, char) in enumerate(line.rstrip()):
        if char.isdigit():
            if len(numbers[-1]) > 0 and numbers[-1][-1][0] == col:
                numbers[-1][-1] = (col + 1, numbers[-1][-1][1] + char)
            else:
                numbers[-1].append((col + 1, char))
        elif char == "*":
            symbols[-1].append(col)
numbers.append([])
for (idx, syms) in enumerate(symbols):
    for sym in syms:
        def matches():
            res = []
            for row in range(idx, idx + 3):
                for (col, num) in numbers[row]:
                    if col - len(num) - 1 <= sym and sym <= col:
                        res.append(num)
            return res
        m = matches()
        if len(m) == 2:
            r += int(m[0]) * int(m[1])
print(r)

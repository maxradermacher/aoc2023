import sys

r = 0
numbers = []
symbols = []
symbols.append([])
for line in sys.stdin:
    numbers.append([])
    symbols.append([])
    for (col, char) in enumerate(line.rstrip()):
        if char.isdigit():
            if len(numbers[-1]) > 0 and numbers[-1][-1][0] == col:
                numbers[-1][-1] = (col + 1, numbers[-1][-1][1] + char)
            else:
                numbers[-1].append((col + 1, char))
        elif char != ".":
            symbols[-1].append(col)
symbols.append([])
for (idx, nums) in enumerate(numbers):
    for (col, num) in nums:
        def matches():
            for row in range(idx, idx + 3):
                for sym in symbols[row]:
                    if col - len(num) - 1 <= sym and sym <= col:
                        return True
            return False
        if matches():
            r += int(num)
print(r)

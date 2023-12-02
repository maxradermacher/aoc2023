import sys

def set_power(rounds):
    m = { "red": 0, "green": 0, "blue": 0 }
    for draws in rounds.split("; "):
        for draw in draws.split(", "):
            num, color = draw.split(" ")
            m[color] = max(m[color], int(num))
    return m["red"] * m["green"] * m["blue"]

r = 0
for l in sys.stdin:
    game, rounds = l.rstrip().split(": ")
    game = int(game[5:])
    r += set_power(rounds)
print(r)

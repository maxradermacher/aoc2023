import sys

M = {
    "red": 12,
    "green": 13,
    "blue": 14,
}

def possible(rounds):
    for draws in rounds.split("; "):
        for draw in draws.split(", "):
            num, color = draw.split(" ")
            if int(num) > M[color]:
                return False
    return True

r = 0
for l in sys.stdin:
    game, rounds = l.rstrip().split(": ")
    game = int(game[5:])
    if possible(rounds):
        r += game
print(r)

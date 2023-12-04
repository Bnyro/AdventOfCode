import math

MAX_MAP = {
    "red": 12,
    "green": 13,
    "blue": 14
}

with open("./inputs/day_2.txt") as f:
    possible_games = []
    powers = []
    for game in f.read().split("\n"):
        if not game:
            continue
        
        game_num = int(game.split(": ")[0].replace("Game ", ""))
        
        invalid = False

        max = {}
        for part in game.split(": ")[1].split("; "):
            puzzles = part.split(", ")
            
            for puzzle in puzzles:
                amount = int(puzzle.split(" ")[0])
                color = puzzle.split(" ")[1]
                
                if MAX_MAP[color] < amount:
                    invalid = True
                
                if max.get(color, 0) < amount:
                    max[color] = amount

        if not invalid:
            possible_games.append(game_num)

        powers.append(math.prod(max.values()))

    print("2_1", sum(possible_games))
    print("2_2", sum(powers))

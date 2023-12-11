import string
file = open("input.txt", "r").read()

lines = file.splitlines()

sets = []
total = 0
totalm = 0

for set in lines:
    gameID = set.split(":")[0]
    games = set.split(":")[1].split(";")
    print(gameID)
    print(games)
    possibility = True
    red = 0
    green = 0
    blue = 0
    for game in games:
        print(game)
        for smallgame in game.split(","):
            if "red" in smallgame:
                print("Found red!: " + smallgame)
                number = int(smallgame.strip(string.ascii_letters).strip(string.punctuation))
                if number > red:
                    red = number
                if number > 12:
                    possibility = False
            if "green" in smallgame:
                print("Found green!: " + smallgame)
                number = int(smallgame.strip(string.ascii_letters).strip(string.punctuation))
                if number > green:
                    green = number
                if number > 13:
                    possibility = False
            if "blue" in smallgame:
                print("Found blue!: " + smallgame)
                number = int(smallgame.strip(string.ascii_letters).strip(string.punctuation))
                if number > blue:
                    blue = number
                if number > 14:
                    possibility = False
    totalm += red * green * blue
    if possibility:
        print("GameID: " + gameID + "\n")
        total += int(gameID.strip(string.whitespace).strip(string.ascii_letters).strip(string.punctuation))

print(total)
print(totalm)
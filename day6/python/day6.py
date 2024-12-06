import copy


def simulate(map, y, x, track_obstacles=False):
    directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    dp = 0
    d = directions[dp]
    path = set()
    path_max = 0
    while True:
        # # Loop detection
        path_len = len(path)
        path.add((y, x))
        if path_len == len(path):
            path_max -= 1
            if path_max <= 0:
                return (len(path), True)
        else:
            path_max = len(path) * 5

        ny = y + d[0]
        nx = x + d[1]
        if 0 > nx or nx >= width or 0 > ny or ny >= height:
            break
        if map[ny][nx] == "#":
            dp = dp + 1 if dp + 1 < len(directions) else 0
            d = directions[dp]
        elif track_obstacles:
            obstacles.add((ny, nx))
        y = y + d[0]
        x = x + d[1]

    return (len(path), False)


if __name__ == "__main__":
    with open("../../inputs/day6.txt") as f:
        data = f.read().strip().split("\n")

    answer1 = 0
    answer2 = 0
    width = len(data[1])
    height = len(data)
    x = y = 0
    for yy in range(height):
        for xx in range(width):
            if data[yy][xx] == "^":
                x = xx
                y = yy

    map = []
    global obstacles
    obstacles = set()
    for _ in range(width):
        map.append(list(data[_]))

    answer1 = simulate(map, y, x, True)[0]

    for o in obstacles:
        if o[0] == y and o[1] == x:
            continue
        new_map = copy.deepcopy(map)
        new_map[o[0]][o[1]] = "#"
        if looped := simulate(new_map, y, x)[1]:
            answer2 += 1

    print(answer1, answer2, sep="\n")

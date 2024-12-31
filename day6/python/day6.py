import copy


def simulate(map, y, x, track_obstacles=False):
    directions = [(-1, 0), (0, 1), (1, 0), (0, -1)]
    d = 0
    path = set()
    path_len = 3
    while True:
        path.add((y, x))
        y = y + directions[d][0]
        x = x + directions[d][1]
        if 0 > x or x >= len(map[0]) or 0 > y or y >= len(map):
            break
        if map[y][x] == "#":
            y = y - directions[d][0]
            x = x - directions[d][1]
            d = (d + 1) % 4
        elif track_obstacles:
            obstacles.add((y, x))

        if (y, x) in path:
            path_len -= 1
        else:
            path_len += 1

        if path_len == 0:
            return (len(path), True)

    return (len(path), False)


if __name__ == "__main__":
    answer1 = 0
    answer2 = 0
    with open("../../inputs/day6.txt") as f:
        data = f.read().strip().split("\n")

    x = y = 0
    for yy in range(len(data)):
        for xx in range(len(data[0])):
            if data[yy][xx] == "^":
                x = xx
                y = yy

    map = []
    global obstacles
    obstacles = set()
    for _ in range(len(data)):
        map.append(list(data[_]))

    answer1 = simulate(map, y, x, True)[0]

    for o in obstacles:
        new_map = copy.deepcopy(map)
        new_map[o[0]][o[1]] = "#"
        if simulate(new_map, y, x)[1]:
            answer2 += 1

    print(answer1, answer2, sep="\n")

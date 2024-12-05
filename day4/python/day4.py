def get(x, y, data):
    if y < 0 or x < 0:
        return " "
    try:
        return data[y][x]
    except IndexError:
        return " "


def count_ones(n, bits):
    count = 0
    mask = 1
    for i in range(0, bits):
        if n & mask:
            count += 1
        mask <<= 1

    return count


if __name__ == "__main__":
    with open("../../inputs/day4.txt") as f:
        data = f.readlines()

    rows = len(data[0])
    cols = len(data)
    match = "XMAS"
    checks = [(-1, 0), (1, 0), (0, -1), (0, 1),
              (-1, -1), (1, -1), (-1, 1), (1, 1)]
    y = 0
    x = 0
    answer1 = 0
    answer2 = 0

    for y in range(0, cols):
        for x in range(0, rows - 1):
            if data[y][x] == "X":
                for (xx, yy) in checks:
                    test_str = ""
                    for i in range(1, len(match)):
                        test_str += get(x+(xx * i), y+(yy * i), data)
                        if get(x+(xx * i), y+(yy * i), data) != match[i]:
                            break
                    else:
                        assert test_str == "MAS"
                        answer1 += 1
            elif data[y][x] == "A":
                tl = get(x - 1, y - 1, data)
                tr = get(x + 1, y - 1, data)
                bl = get(x - 1, y + 1, data)
                br = get(x + 1, y + 1, data)

                for v in [tl, tr, bl, br]:
                    if v == "S" or v == "M":
                        continue
                    else:
                        break
                else:
                    if tl != br and tr != bl:
                        answer2 += 1

    print(answer1, answer2, sep="\n")

def count(input, match_value, map: dict):
    # If we've already seen this value, return it's count from map
    if stored_value := map.get(match_value, False):
        return stored_value

    count = 0
    try:
        i = input.index(match_value)
    except ValueError:
        return 0

    # input has to be sorted, check at first index of match_value, stop
    # checking once value changes
    while input[i] == match_value:
        count += 1
        i += 1
    map[match_value] = count
    return count


if __name__ == "__main__":
    with open("../../inputs/day1.txt") as file:
        lines = file.readlines()

    l1 = []
    l2 = []
    answer1 = 0
    answer2 = 0
    for line in lines:
        nums = line.split("   ")
        l1.append(int(nums[0]))
        l2.append(int(nums[-1]))

    l1.sort()
    l2.sort()
    count_map = {}
    for n1, n2 in zip(l1, l2):
        answer1 += abs(n1 - n2)
        answer2 += n1 * count(l2, n1, count_map)

    print(answer1, answer2, sep="\n")

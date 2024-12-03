def str_list_to_int_list(input):
    return [int(v) for v in input]


def error_count(input):
    increasing = True
    errors = 0
    i = 0
    while i < len(input) - 1:
        l1 = input[i]
        l2 = input[i + 1]
        if l1 > l2 and i == 0:
            increasing = False

        if ((l1 < l2 and not increasing) or (l1 > l2 and increasing)
           or abs(l1 - l2) > 3 or abs(l1 - l2) <= 0):
            errors += 1

        i += 1

    return errors


if __name__ == "__main__":
    with open("../../inputs/day2.txt") as f:
        lines = f.readlines()

    data = [str_list_to_int_list(line.split(" ")) for line in lines]
    answer1 = 0
    answer2 = 0
    for entry in data:
        if error_count(entry) == 0:
            answer1 += 1
            answer2 += 1
            continue
        for i in range(0, len(entry)):
            if error_count(entry[0:i] + entry[i+1:len(entry)]) == 0:
                answer2 += 1
                break

    print(answer1, answer2, sep="\n")

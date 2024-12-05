def fix_invalid(update, rules):
    new_update = list(update)
    for rule in rules:
        if is_valid(update, [rule]):
            continue
        else:
            tmp0 = update[update.index(rule[1])]
            tmp1 = update[update.index(rule[0])]
            new_update[update.index(rule[0])] = tmp0
            new_update[update.index(rule[1])] = tmp1
            return fix_invalid(tuple(new_update), rules)

    return tuple(new_update)


def is_valid(update, rules):
    for rule in rules:
        if ((rule[0] not in update or rule[1] not in update)
           or rule[1] in update[update.index(rule[0]):]):
            continue
        else:
            return False

    return True


def get_mid(input):
    return input[int(len(input) / 2)]


if __name__ == "__main__":
    with open("../../inputs/day5.txt") as file:
        rules_str, updates_str = file.read().strip().split("\n\n")
    answer1 = 0
    answer2 = 0
    rules = []
    updates = []
    for v in rules_str.split("\n"):
        rules.append(tuple(map(int, v.split("|"))))

    for v in updates_str.split("\n"):
        updates.append(tuple(map(int, v.split(","))))

    for update in updates:
        if is_valid(update, rules):
            answer1 += get_mid(update)
        else:
            fixed = fix_invalid(update, rules)
            answer2 += get_mid(fixed)

    print(answer1, answer2, sep="\n")

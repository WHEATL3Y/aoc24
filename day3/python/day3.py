import re

if __name__ == "__main__":
    answer1 = 0
    answer2 = 0
    with open("../../inputs/day3.txt") as f:
        data = f.read()

    do = True
    matches = re.findall(r"mul\(\d{1,3},\d{1,3}\)|do\(\)|don't\(\)", data)
    for entry in matches:
        if entry == "do()":
            do = True
            continue
        elif entry == "don't()":
            do = False
            continue

        nums = re.findall(r"\d{1,3}", entry)
        product = int(nums[0]) * int(nums[1])
        answer1 += product
        if do:
            answer2 += product

    print(answer1, answer2, sep="\n")

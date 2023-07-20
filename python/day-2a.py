shapes = {
    "A": 0,
    "B": 1,
    "C": 2,

    "X": 0,
    "Y": 1,
    "Z": 2
}


def get_score(elf, you):
    score = 1 + shapes[you]
    if shapes[elf] == shapes[you]:
        return score + 3
    if (shapes[elf] + 1) % 3 == shapes[you]:
        return score + 6
    return score


def main():
    with open('day-2a/input.txt', 'r') as f:
        total_score = 0
        for line in f.readlines():
            total_score += get_score(line[0], line[2])
        return total_score


print(main())

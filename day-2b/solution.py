shapes = {
    "A": 0,
    "B": 1,
    "C": 2
}


def get_score(elf, you):
    if you == 'Y':
        return shapes[elf] + 4

    if you == 'Z':
        return (shapes[elf] + 1) % 3 + 7

    return (shapes[elf] - 1) % 3 + 1


def main():
    with open('day-2a/input.txt', 'r') as f:
        total_score = 0
        for line in f.readlines():
            total_score += get_score(line[0], line[2])
        return total_score


print(main())

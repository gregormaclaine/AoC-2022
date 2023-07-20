def get_priority(char):
    val = ord(char)
    if val > 96:
        return val - 96
    return val - 65 + 27


def main():
    with open('input/day-3.txt', 'r') as f:
        total = 0

        for line in f.readlines():
            if line.endswith('\n'):
                line = line[:-1]
            letters = {}
            mid_point = int(len(line) / 2)

            for c in line[:mid_point]:
                letters[c] = True

            for c in line[mid_point:]:
                if c in letters:
                    # print(c, get_priority(c))
                    total += get_priority(c)
                    break

        return total


print(main())

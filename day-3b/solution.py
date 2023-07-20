def get_priority(char):
    val = ord(char)
    if val > 96:
        return val - 96
    return val - 65 + 27


def main():
    with open('day-3/input.txt', 'r') as f:
        total = 0

        lines = f.read().split('\n')
        for i in range(0, len(lines), 3):
            letters1 = {}
            letters2 = {}

            for c in lines[i]:
                letters1[c] = True

            for c in lines[i + 1]:
                letters2[c] = True

            for c in lines[i + 2]:
                if c in letters1 and c in letters2:
                    # print(c, get_priority(c))
                    total += get_priority(c)
                    break

        return total


print(main())

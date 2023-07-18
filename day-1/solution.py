import timeit


def main():
    with open('day-1/input.txt', 'r') as f:
        elves = []
        running_total = 0
        for line in f.readlines():
            if line == '\n':
                elves.append(running_total)
                running_total = 0
            else:
                running_total += int(line[:-1])

        max_i = 0
        for i in range(1, len(elves)):
            if (elves[i] > elves[max_i]):
                max_i = i

        return elves[max_i]


print(timeit.timeit(main, number=1000))

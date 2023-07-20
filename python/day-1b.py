def main():
    with open('input/day-1.txt', 'r') as f:
        elves = [sum(map(int, seq.split('\n')))
                 for seq in f.read().split('\n\n')]
        elves.sort(reverse=True)
        return sum(elves[:3])


print(main())

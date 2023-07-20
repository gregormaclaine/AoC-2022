def main():
    with open('input/day-1.txt', 'r') as f:
        return max(sum(map(int, seq.split('\n')))
                   for seq in f.read().split('\n\n'))


print(main())

import sys
from functools import lru_cache

@lru_cache(maxsize=None)
def count_digits(num):
    if num == 0:
        return 1
    return len(str(num))

@lru_cache(maxsize=None)
def split_in_half(num):
    s = str(num)
    mid = len(s) // 2
    return ' '.join([str(int(s[:mid])), str(int(s[mid:]))])

@lru_cache(maxsize=None)    
def convert(stone):
    if stone == '0':
        result = '1'
    elif len(stone) % 2 == 0:
        result = split_in_half(stone)
    else:
        result = str(int(stone) * 2024)
    return result
    
@lru_cache(maxsize=None)
def blink_stone(stone, iterations):
    converted = convert(stone)
    if (iterations > 1):
        return sum(blink_stone(c, iterations - 1) for c in converted.split(' '))
    else:
        return len(converted.split(' '))

def main(argv):
    with open(argv[1], 'r') as f:
        stones = [int(s) for s in f.readline().strip().split(' ')]
        stones.sort()
        total = 0
        for num in stones:
            total += blink_stone(str(num), 75)
            print()
        print(total)

if __name__ == '__main__':
    main(sys.argv)

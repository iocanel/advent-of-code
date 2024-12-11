import sys


def count_digits(num):
    count = 0
    if num == 0:
        return 1
    while num > 0:
        count += 1
        num //= 10
    return count

def split_in_half(num):
    s = str(num)
    return int(s[:len(s) // 2]), int(s[len(s) // 2:])

print(split_in_half(1000))    

def convert(stone):
    if stone == 0:
        return [1]
    elif count_digits(stone) % 2 == 0:
        return split_in_half(stone)
    else:
        return [stone * 2024]
    
def blink(stones):
    result = []
    for x in range(len(stones)):
        for c in convert(stones[x]):
            result.append(c)
    return result

def main(argv):
    with open(argv[1], 'r') as f:
        stones = [int(s) for s in f.readline().strip().split(' ')]
        for _ in range(25):
            stones = blink(stones)
        print(len(stones))

if __name__ == '__main__':
    main(sys.argv)

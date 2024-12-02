import sys

def as_number(str):
    numbers = {
        "one": 1,
        "two": 2,
        "three": 3,
        "four": 4,
        "five": 5,
        "six": 6,
        "seven": 7,
        "eight": 8,
        "nine": 9,
        "0": 0
    }
    for word, num in numbers.items():
        if str.lower().endswith(word):
            return num
    return -1

def line_to_number(line):
    first_digit = -1 
    last_digit = -1
    for index, char in enumerate(line):
            is_digit = char.isdigit()
            if is_digit:
                if first_digit < 0:
                    first_digit = int(char)
                last_digit = int(char)
            else:
                sub_str = line[0:index+1]
                num = as_number(sub_str)
                if num >= 0:
                    if first_digit < 0:
                        first_digit = num
                    last_digit = num
    num = (10 * first_digit + last_digit)
    return num


with open(sys.argv[1]) as f:
    lines = f.readlines()
    sum = 0
    for line in lines:
        num = line_to_number(line)
        sum += num
    print(sum)


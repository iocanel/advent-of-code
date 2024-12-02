import sys

with open(sys.argv[1]) as f:
    lines = f.readlines()

    sum = 0
    for line in lines:
        first_digit = -1 
        last_digit = -1
        for char in line:
            is_digit = char.isdigit()
            if is_digit:
                if first_digit < 0:
                    first_digit = int(char)
                last_digit = int(char)
        num = (10 * first_digit + last_digit)
        sum += num
    print(sum)


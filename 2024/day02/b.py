import sys


safe = 0

def is_safe(row):
    difs = [row[i] - row[i - 1] for i in range(1, len(row))]
    if all([1 <= abs(dif) <= 3 for dif in difs]) and (all([dif > 0 for dif in difs]) or all([dif < 0 for dif in difs])):
        return True
    return False

# Read the input file
with open(sys.argv[1], 'r') as file:
    lines = file.readlines()
    for line in lines:
        report = [int(x) for x in line.split()]
        if is_safe(report):
            safe += 1
            continue

        for i in range(0, len(report)):
            candidate = report[:i] + report[i+1:]
            if is_safe(candidate):
                safe += 1
                print(report, '->', candidate)
                break

    print(safe)

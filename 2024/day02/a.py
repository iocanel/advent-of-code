import sys


safe = 0

# Read the input file
with open(sys.argv[1], 'r') as file:
    lines = file.readlines()
    for line in lines:
        report = [int(x) for x in line.split()]
        difs = [report[i] - report[i - 1] for i in range(1, len(report))]
        # check if the differences are in the range of 1 to 3
        if not all([1 <= abs(dif) <= 3 for dif in difs]):
            print(report, ' diffs:', difs)
        elif not (all([dif > 0 for dif in difs]) or all([dif < 0 for dif in difs])):
            print(report, ' diffs:', difs)
        else:
            safe += 1
    print(safe)

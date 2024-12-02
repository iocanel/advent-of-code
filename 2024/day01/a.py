import sys

left = []
right = []

with open(sys.argv[1]) as f:
    lines = f.readlines()
    for line in lines:
        left.append(int(line.split()[0]))
        right.append(int(line.split()[1]))

left.sort()
right.sort()

result = 0

for i in range(len(left)):
    result += abs(left[i] - right[i])

print(result)

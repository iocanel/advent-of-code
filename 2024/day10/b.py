import sys


def find_trailheads(map):
    trailheads = []
    for x in range(len(map)):
        for y in range(len(map[x])):
            if map[x][y] == 0:
                trailheads.append([x, y])
    return trailheads

def find_trail_ends(map, x, y):
    if map[x][y] == 9:
        return [[x, y]]
    trail_ends = []
    if x < len(map) - 1 and map[x + 1][y] == map[x][y] + 1:
        d_ends = find_trail_ends(map, x + 1, y)
        for e in d_ends:
            trail_ends.append(e)
    if x > 0 and map[x - 1][y] == map[x][y] + 1:
        u_ends = find_trail_ends(map, x - 1, y)
        for e in u_ends:
            trail_ends.append(e)
    if y < len(map[x]) - 1 and map[x][y + 1] == map[x][y] + 1:
        r_ends = find_trail_ends(map, x, y + 1)
        for e in r_ends:
             trail_ends.append(e)
    if y > 0 and map[x][y - 1] == map[x][y] + 1:
        l_ends = find_trail_ends(map, x, y - 1)
        for e in l_ends:
             trail_ends.append(e)
    return trail_ends

def count_all_trails(map):
    count = 0
    for x in range(len(map)):
        for y in range(len(map[x])):
            if map[x][y] == 0:
                trail_ends = find_trail_ends(map, x, y)
                print(x, y, trail_ends)
                count += len(trail_ends)
    return count


def main(argv):
    with open(argv[1], 'r') as f:
        map = [[int(char) for char in line.strip()] for line in f]
        print(map)
        print(count_all_trails(map))

if __name__ == '__main__':
    main(sys.argv)

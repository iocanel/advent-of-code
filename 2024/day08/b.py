import sys

def has_frequency(arr):
    for x in arr:
        if x.isdigit():
            return True
        if x.isalpha():
            return True
    return False

def distance(x1, y1, x2, y2):
    return ((x1 - x2) ** 2 + (y1 - y2) ** 2) ** 0.5

def slope(x1, y1, x2, y2):
    if x2 - x1 == 0:
        return 0
    return (y2 - y1) / (x2 - x1)

def create_frequency_map(table):
    freq_map = {}
    for x in range(len(table)):
        for y in range(len(table[x])):
            if has_frequency(table[x][y]):
                freq_map.setdefault(table[x][y], []).append([x, y])
    return freq_map

def find_antinodes(table):
    result = [row[:] for row in table]
    freq_map = create_frequency_map(table)
    print(freq_map)
    for x in range(len(table)):
        for y in range(len(table[x])):
            if has_frequency(table[x][y]) and len(freq_map[table[x][y]]) > 1:
                print([x, y], table[x][y])
                result[x][y] = '#'
                continue
            for key, value in freq_map.items():
                for p1 in value:
                    s1 = slope(x, y, p1[0], p1[1])
                    for p2 in value:
                        if p1 == p2:
                            continue
                        s2 = slope(x, y, p2[0], p2[1])
                        s3 = slope(p1[0], p1[1], p2[0], p2[1])
                        if s1 == s2 == s3: 
                            print([x, y], key, s1, s2, s3)
                            result[x][y] = '#'
    return result 

def count_antinodes(table):
    antinodes = find_antinodes(table)
    count = 0
    for x in range(len(antinodes)):
        for y in range(len(antinodes[x])):
            if antinodes[x][y] == '#':
                count += 1
    return count

def print_map(table):
    for row in table:
        print(''.join(row))

def main(filename):
    with open(filename) as f:
        map = [list(line.strip()) for line in f]
        antinodes = find_antinodes(map)
        print_map(antinodes)
        print(count_antinodes(map))

if __name__ == '__main__':
    main(sys.argv[1])

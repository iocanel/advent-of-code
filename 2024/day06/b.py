import sys
import copy

def turn_left(symbol):
    if symbol == '^':
        return '<'
    if symbol == '<':
        return 'v'
    if symbol == 'v':
        return '>'
    if symbol == '>':
        return '^'

def turn_right(symbol):
    if symbol[-1] == '^':
        return '>'
    if symbol[-1] == '>':
        return 'v'
    if symbol[-1] == 'v':
        return '<'
    if symbol[-1] == '<':
        return '^'

def find_guard_coords(map):
    for x, line in enumerate(map):
        for y, symbol in enumerate(line):
            if symbol[-1] in ['^', 'v', '<', '>']:
                return x, y
    raise ValueError('Guard not found')

def find_next_step(map, x, y):
    symbol = map[x][y]
    if symbol[-1] == '^':
        return x - 1, y
    if symbol[-1]  == 'v':
        return x + 1, y
    if symbol[-1] == '>':
        return x, y + 1
    if symbol[-1] == '<':
        return x, y - 1
    raise ValueError('Illegal state. Unknown guard symbol')


def move_guard(map, gx, gy):    
    nx, ny = find_next_step(map, gx, gy)
    if nx < 0 or nx >= len(map) or ny < 0 or ny >= len(map[0]):
        return nx, ny, map[gx][gy][-1]
    elif map[nx][ny] == '#':
        return gx, gy, turn_right(map[gx][gy])
    else:
        return nx, ny, map[gx][gy][-1]

def patrol(map):
    x, y = find_guard_coords(map)
    while x >= 0 and x < len(map) and y >= 0 and y < len(map[0]):
        symbol = map[x][y]
        nx, ny, new_symbol = move_guard(map, x, y)
        if nx < 0 or nx >= len(map) or ny < 0 or ny >= len(map[0]):
            return True
        if new_symbol in map[nx][ny]:
            return False
        map[nx][ny] += new_symbol
        x, y = nx, ny
    return True
        

def count_steps(map):
    steps = 1
    for x, line in enumerate(map):
        for y, symbol in enumerate(line):
            if map[x][y] == 'X':
                steps += 1
    return steps    
    
def main(filename):
    with open(filename) as f:
        options = 0
        original_map = [list(line.strip()) for line in f]
        gx, gy = find_guard_coords(original_map)
        for x, line in enumerate(original_map):
            for y, symbol in enumerate(line):
                if gx == x and gy == y:
                    continue
                elif symbol == '#':
                    continue
                map = copy.deepcopy(original_map)
                map[x][y] = '#'
                if not patrol(map):
                    options += 1
    print(options)


if __name__ == '__main__':
    main(sys.argv[1])

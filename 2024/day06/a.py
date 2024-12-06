import sys


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
    if symbol == '^':
        return '>'
    if symbol == '>':
        return 'v'
    if symbol == 'v':
        return '<'
    if symbol == '<':
        return '^'

def find_guard_coords(map):
    for x, line in enumerate(map):
        for y, symbol in enumerate(line):
            if symbol in ['^', 'v', '<', '>']:
                return x, y
    raise ValueError('Guard not found')

def find_next_step(map, x, y):
    symbol = map[x][y]
    if symbol == '^':
        return x - 1, y
    if symbol == 'v':
        return x + 1, y
    if symbol == '>':
        return x, y + 1
    if symbol == '<':
        return x, y - 1
    raise ValueError('Illegal state. Unknown guard symbol')


def move_guard(map, gx, gy):    
    nx, ny = find_next_step(map, gx, gy)
    if nx < 0 or nx >= len(map) or ny < 0 or ny >= len(map[0]):
        return nx, ny
    elif map[nx][ny] == '#':
        map[gx][gy] = turn_right(map[gx][gy])
        return gx, gy
    else:
        map[nx][ny] = map[gx][gy]
        map[gx][gy] = 'X'
        return nx, ny

def patrol(map):
    x, y = find_guard_coords(map)
    while x >= 0 and x < len(map) and y >= 0 and y < len(map[0]):
        x, y = move_guard(map, x, y)

def count_steps(map):
    steps = 1
    for x, line in enumerate(map):
        for y, symbol in enumerate(line):
            if map[x][y] == 'X':
                steps += 1
    return steps    
    
def main(filename):
    with open(filename) as f:
        map = [list(line.strip()) for line in f]
        patrol(map)
        steps = count_steps(map)
        print(map)
        print(steps)


if __name__ == '__main__':
    main(sys.argv[1])

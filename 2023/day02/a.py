import sys
import re

contents = {
    'red': 12,
    'green': 13,
    'blue': 14
}
def find_game_id(line):
    parts = re.split(r'[ :]', line)
    return int(parts[1])

def get_colors(line):
    parts = re.split(r'[ :,;\n]', line)
    colors = {}
    for index  in range(2, len(parts) - 1):
        c = parts[index]
        value = parts[index - 1]
        if not c in ['red', 'green', 'blue']:
            continue
        if colors.__contains__(c):
            colors[c] = max (int(colors[c]), int(value))
        else:
            colors[c] = int(value)
    return colors


def is_possible(line):
    line_colors = get_colors(line)
    for color in line_colors:
        if contents[color] < line_colors[color]:
            print ('color:', color, '[', contents[color], '<',  line_colors[color] ,']', line)
            return False
    return True

with open(sys.argv[1], 'r') as f:
    lines = f.readlines()
    sum = 0
    for line in lines:
        if is_possible(line):
            sum += int(find_game_id(line))
#            print(find_game_id(line), get_colors(line), line)
        #else:
        #    print(find_game_id(line), get_colors(line), line)
    print(sum)


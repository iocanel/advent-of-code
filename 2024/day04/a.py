import sys


def read_table(filename):
    # Read the file into a two dimensional array of characters
    with open(filename, 'r') as f:
        lines = f.readlines()
        return [list(line.strip()) for line in lines]

table = read_table(sys.argv[1])

def print_table(table):
    for i in range(len(table)):
        for j in range(len(table[i])):
            print(table[i][j], end='')
        print()

def diagonal_d(table, row, col):
    result = [' ' for i in range(len(table) - (max(col, row)))]
    for i in range(len(result)):
        result[i] = table[row + i][col + i]
    return ''.join(result)

def diagonal_u(table, row, col):
    result = [' ' for i in range((min(row + 1, len(table) - col)))]
    for i in range(len(result)):
        result[i] = table[row - i][col + i]
    return ''.join(result)

def remaining_row(table, row, col):
    return ''.join(table[row][col:])

def remaining_col(table, row, col):
    return ''.join([table[i][col] for i in range(row, len(table))])


def count(table, word):
    count = 0
    for i in range(len(table)):
        for j in range(len(table[i])):
            if table[i][j] == word[0]:
                if word in ''.join(table[i][j:]):
                    count += 1
    return count

def count_diagonal(table, word):
    count = 0
    for i in range(len(table)):
        for j in range(len(table[i])):
            if table[i][j] == word[0]:
                if word in ''.join(diagonal_d(table, j, i)):
                    count += 1
                if word in ''.join(diagonal_u(table, j, i)):
                    count += 1
    return count

def count_occurances(table, word):
    count = 0
    reversed_word = ''.join(reversed(word))
    dotted = [['.' for j in range(len(table[0]))] for i in range(len(table))]
    for i in range(len(table)):
        for j in range(len(table[i])):
             row = remaining_row(table, i, j)
             col = remaining_col(table, i, j)
             diag_d = diagonal_d(table, i, j)
             diag_u = diagonal_u(table, i, j)
             if row.startswith(word) or row.startswith(reversed_word):
                 dotted[i][j] = table[i][j]
                 dotted[i][j + 1] = table[i][j + 1]
                 dotted[i][j + 2] = table[i][j + 2]
                 dotted[i][j + 3] = table[i][j + 3]
                 count += 1
             if col.startswith(word) or col.startswith(reversed_word):
                 dotted[i][j] = table[i][j]
                 dotted[i + 1][j] = table[i + 1][j]
                 dotted[i + 2][j] = table[i + 2][j]
                 dotted[i + 3][j] = table[i + 3][j]
                 count += 1
             if diag_d.startswith(word) or diag_d.startswith(reversed_word):
                 dotted[i][j] = table[i][j]
                 dotted[i + 1][j + 1] = table[i + 1][j + 1]
                 dotted[i + 2][j + 2] = table[i + 2][j + 2]
                 dotted[i + 3][j + 3] = table[i + 3][j + 3]
                 count += 1
             if diag_u.startswith(word) or diag_u.startswith(reversed_word):
                 dotted[i][j] = table[i][j]
                 dotted[i - 1][j + 1] = table[i - 1][j + 1]
                 dotted[i - 2][j + 2] = table[i - 2][j + 2]
                 dotted[i - 3][j + 3] = table[i - 3][j + 3]
                 count += 1
    print_table(dotted)
    return count

print_table(table)
counter = count_occurances(table, 'XMAS')
print(counter)

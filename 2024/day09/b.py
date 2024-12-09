import sys

def diskmap_to_blocks(diskmap):  
    blocks = []
    for index, c in enumerate(diskmap):
        if index % 2 == 0:
            for i in range(int(c)):
                blocks.append(str(int(index / 2)))
        else:
            for i in range(int(c)):
                blocks.append('.')
    return blocks


def next_gap(blocks, size):
    for i in range(len(blocks)):
        if blocks[i] == '.':
            j = i
            while j < len(blocks) and blocks[j] == '.':
                j += 1
            if j - i >= size:
                return i, j 
    return -1, -1

def compact(blocks):
    result = [x for x in blocks]
    i = len(result) - 1
    while i > 0:
        current = result[i]
        print(i, result[i])
        if result[i] != '.':
            j = i
            while j > 0 and result[j] == result[i]:
                j -= 1
            b_size = i - j
            g_begin, g_end = next_gap(result[:i], b_size)
            print(current, b_size, g_begin, g_end)
            if g_begin == -1:
                i -= b_size
                continue
            for j in range(b_size):
                result[g_begin + j] = current
                result[i - j] = '.'
            print(''.join(result))
            i -= b_size
        else:
            i -= 1
    return result

def checksum(blocks):
    result = 0
    for index, file_id in enumerate(blocks):
        if file_id == '.':
            continue
        result += int(file_id) * index
    return result

def main(input):
    print(input)
    diskmap = diskmap_to_blocks(list(input))
    comp = compact(diskmap)
    checksum_value = checksum(comp)
    print(''.join(diskmap))
    print(''.join(comp))
    print(checksum_value)

if __name__ == '__main__':
    with open(sys.argv[1], 'r') as f:
        input = f.readline().strip()
        main(input)

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


def next_gap(blocks):
    for i in range(len(blocks)):
        if blocks[i] == '.':
            return i
    return -1

def compact(blocks):
    result = [x for x in blocks]
    for i in range(len(result) - 1, 0, -1):
        if result[i] != '.':
            gap = next_gap(result[:i])
            if gap == -1:
                return result
            result[gap] = result[i]
            result[i] = '.'
    return result

def checksum(blocks):
    result = 0
    for index, file_id in enumerate(blocks):
        if file_id == '.':
            break
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

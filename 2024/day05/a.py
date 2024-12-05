import sys

def find_middle_page(update):
    return update[len(update) // 2]

def is_update_valid(update, page_ordering_rules):
    for rule in page_ordering_rules:
        first, second = rule
        try:
            first_index = update.index(first)
            second_index = update.index(second)
        except ValueError:
            continue
        if first_index > second_index:
            return False
    return True

def main(file_path):
    with open(file_path) as f:
        lines = f.readlines()

    page_ordering_rules = [[int(v) for v in line.strip().split('|')] for line in lines if '|' in line]
    updates = [[int(v) for v in line.strip().split(',')] for line in lines if ',' in line]

    total_sum = 0
    for update in updates:
        if is_update_valid(update, page_ordering_rules):
            page =  find_middle_page(update)
            total_sum += page
    print(total_sum)

if __name__ == '__main__':
    main(sys.argv[1])



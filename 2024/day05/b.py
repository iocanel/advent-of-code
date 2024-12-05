from os import truncate
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


def compare(a, b, page_ordering_rules):
    for rule in page_ordering_rules:
        if rule[0] == a and rule[1] == b:
            return False
        if rule[0] == b and rule[1] == a:
            return True 
    return True

def sort(array, page_ordering_rules):
    result = [i for i in array]
    for i in range(len(result)):
        for j in range(i, len(result)):
            if compare(result[i], result[j], page_ordering_rules):
                result[i], result[j] = result[j], result[i]
    return result

def main(file_path):
    with open(sys.argv[1]) as f:
        lines = f.readlines()
    
    page_ordering_rules = [[int(v) for v in line.strip().split('|')] for line in lines if '|' in line]
    updates = [[int(v) for v in line.strip().split(',')] for line in lines if ',' in line]
    sum = 0
    for update in updates:
        if not is_update_valid(update, page_ordering_rules):
            sorted_update = sort(update, page_ordering_rules)
            page =  find_middle_page(sorted_update)
            sum += page
    print(sum)

if __name__ == '__main__':    
    main(sys.argv[1])

import sys

with open(sys.argv[1]) as f:
    lines = f.readlines()

page_ordering_rules = [[int(v) for v in line.strip().split('|')] for line in lines if '|' in line]
updates = [[int(v) for v in line.strip().split(',')] for line in lines if ',' in line]

#print(page_ordering_rules)                    
#print(updates)

def find_middle_page(update):
    return update[len(update) // 2]


def is_update_valid(update, page_ordering_rules):
    for rule in page_ordering_rules:
        first = rule[0]
        second = rule[1]
        first_index = -1 
        second_index = -1
        if first in update:
            first_index = update.index(first)
        if second in update:
            second_index = update.index(second)
        if first_index > second_index and second_index != -1 and first_index != -1:
            #print(f'Invalid update: {update} violates rule {rule}')
            return False
    return True


sum = 0
for update in updates:
    if is_update_valid(update, page_ordering_rules):
        page =  find_middle_page(update)
        #print(f'Middle Page: {page}')
        sum += page
print(sum)


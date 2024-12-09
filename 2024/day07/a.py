import sys

def to_num(s):
    return int(''.join([x for x in s if x.isdigit()]))

def min_ops(n):
    """
    The operation number (the number that represents the operations array) that can be performed on n numbers
    0 -> + 
    1 -> *
    2 -> + +    
    3 -> + *
    4 -> * +
    5 -> * *
    """
    return ['+' for i in range(n)]

def next_ops(ops):
    for i in range(len(ops)):
        if ops[i] == '+':
            ops[i] = '*'
            break
        else:
            ops[i] = '+'

def solve(equation, ops):
    result = equation[1]
    for i in range(len(ops)):
        if ops[i] == '+':
            result += equation[i + 2]
        elif ops[i] == '*':
            result *= equation[i + 2]
    return result

def print_equation(equation, ops):            
    output = [equation[0], '=']
    for x, y in zip(equation[1:], ops):
        output.extend([x, y])
    output.append(equation[-1])
    output.append('=')
    output.append(solve(equation, ops))
    print(output)

def is_solvable(equation):
    numbers  = len(equation) - 1
    iterations = 2 ** (numbers - 1)
    ops = min_ops(numbers - 1)
    for n in range(iterations):
        expected = equation[0]
        result = solve(equation, ops)
        if expected == result:
            print_equation(equation, ops)
            return True
        next_ops(ops)
    return False

def main(argv):
    with open(argv[1], 'r') as f:
        lines = f.readlines()
        result = 0
        for line in lines:
            equation = [to_num(x) for x in line.split()]
            if is_solvable(equation):
                result += equation[0]
        print(result)

 
if __name__ == '__main__':
    main(sys.argv)

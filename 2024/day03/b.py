import sys

valid = {'m', 'u', 'l', 'd', 'o', 'n', 't', '\'', '(', ',' , ')', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' }

states = {
    "initial": 0,
    "m": 1,
    "u": 2,
    "l": 3,
    "mul(": 4,
    ",": 5,
    "mul)": 6,
    "d": 7,
    "o": 8,
    "n": 9,
    "'": 10,
    "t": 11,
    "do(": 12,
    "do)": 13,
    "do_not(": 14,
    "do_not)": 15,
    "0": 16,
    "1": 16,
    "2": 16,
    "3": 16,
    "4": 16,
    "5": 16,
    "6": 16,
    "7": 16,
    "8": 16,
    "9": 16,
}
# second digit -> + 1
# third digit -> + 2
# second number -> + 10
# second number second digit -> + 11
# second number third digit -> + 12


def next_state(state, char):
    if not char in valid:
        return 0;
    if state == states["initial"]:
        if char == "m":
            return states["m"]
        elif char == "d":
            return states["d"]
        else:
            return 0
    if state == states["m"]:
        if char == "u":
            return states["u"]
        else:
            return 0
    if state == states["u"]:
        if char == "l":
            return states["l"]
        else:
            return 0
    if state == states["l"]:
        if char == "(":
            return states["mul("]
        else:
            return 0
    if state == states["mul("]:
        if char.isdigit():
            return states["0"]
        else:
            return 0
    if state == states["0"]:
        if char.isdigit():
            return states["0"] + 1
        elif char == ",":
            return states[","]
        else:
            return 0
    if state == states["0"] + 1:
        if char.isdigit():
            return states["0"] + 2
        elif char == ",":
            return states[","]
        else:
            return 0
    if state == states["0"] + 2:
        if char == ",":
            return states[","]
        else:
            return 0
    if state == states[","]:
        if char.isdigit():
            return states["0"] + 10
        else:
            return 0
    if state == states["0"] + 10:
        if char.isdigit():
            return states["0"] + 11
        elif char == ")":
            return states["mul)"]
        else:
            return 0
    if state == states["0"] + 11:
        if char.isdigit():
            return states["0"] + 12
        elif char == ")":
            return states["mul)"]
        else:
            return 0
    if state == states["0"] + 12:
        if char == ")":
            return states["mul)"]
        else:
            return 0
    if state == states["d"]:
        if char == "o":
            print('do')
            return states["o"]
        else:
            return 0
    if state == states["o"]:
        if char == "(":
            return states["do("]
        elif char == "n":
            return states["n"]
        else:
            return 0
    if state == states["do("]:
        if char == ")":
            return states["do)"]
        else:
            return 0
    if state == states["n"]:
        if char == "'":
            return states["'"]
        else:
            return 0
    if state == states["'"]:
        if char == "t":
            return states["t"]
        else:
            return 0
    if state == states["t"]:
        if char == "(":
            return states["do_not("]
        else:
            return 0
    if state == states["do_not("]:
        if char == ")":
            return states["do_not)"]
        else:
            return 0

def evaluate(content):
    state = states["initial"]
    first = 0
    second = 0
    result = 0
    do = True
    for char in content:
        state = next_state(state, char)
        if state == states["0"]:
            first = int(char)
        if state == states["0"] + 1:
            first = first * 10 + int(char)
        if state == states["0"] + 2:
            first = first * 10 + int(char)
        if state == states["0"] + 10:
            second = int(char)
        if state == states["0"] + 11:
            second = second * 10 + int(char)
        if state == states["0"] + 12:
            second = second * 10 + int(char)
        if state == states["mul)"]:
            if do:
                result += first * second
                print('do mul' + str(first) + ' ' + str(second))
            else:
                print('dont mul' + str(first) + ' ' + str(second))
            state = states["initial"]
            first = 0
            second = 0
        if state == states["do)"]:
            do = True
            state = states["initial"]
            first = 0
            second = 0
            print('do')
        if state == states["do_not)"]:
            do = False
            state = states["initial"]
            first = 0
            second = 0
            print('dont')
        if state == states["initial"]:
            first = 0
            second = 0
    return result


with open(sys.argv[1]) as f:
    content = f.read()

print(evaluate(content))

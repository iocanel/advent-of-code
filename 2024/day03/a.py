import sys

valid = {'m', 'u', 'l',  '(', ',' , ')', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9' }

states = {
    "initial": 0,
    "m": 1,
    "u": 2,
    "l": 3,
    "(": 4,
    ",": 5,
    ")": 6,
    "0": 7,
    "1": 7,
    "2": 7,
    "3": 7,
    "4": 7,
    "5": 7,
    "6": 7,
    "7": 7,
    "8": 7,
    "9": 7,
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
            return states["("]
        else:
            return 0
    if state == states["("]:
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
            return states[")"]
        else:
            return 0
    if state == states["0"] + 11:
        if char.isdigit():
            return states["0"] + 12
        elif char == ")":
            return states[")"]
        else:
            return 0
    if state == states["0"] + 12:
        if char == ")":
            return states[")"]
        else:
            return 0

def evaluate(content):
    state = states["initial"]
    first = 0
    second = 0
    result = 0
    for char in content:
        state = next_state(state, char)
        if state == states["0"]:
            print('f1:', char)
            first = int(char)
        if state == states["0"] + 1:
            print('f2:', char)
            first = first * 10 + int(char)
        if state == states["0"] + 2:
            print('f3:', char)
            first = first * 10 + int(char)
        if state == states["0"] + 10:
            print('s1:', char)
            second = int(char)
        if state == states["0"] + 11:
            print('s2:', char)
            second = second * 10 + int(char)
        if state == states["0"] + 12:
            print('s3:', char)
            second = second * 10 + int(char)
        if state == states[")"]:
            result += first * second
            state = states["initial"]
            first = 0
            second = 0
        if state == states["initial"]:
            first = 0
            second = 0
    return result


with open(sys.argv[1]) as f:
    content = f.read()

#print(evaluate("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"))
print(evaluate(content))

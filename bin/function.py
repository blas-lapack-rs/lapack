import re

argument_re = re.compile('\s*(\w+): ([^,\)]+)([,\)\s]*)')
name_re = re.compile('\s*pub fn (?:LAPACKE_)?(\w+[a-z0-9])(_?)')
return_re = re.compile('(?:\s*->\s*([^;]+))?')


class Function():

    def __init__(self, name, args, ret):
        self.name = name
        self.args = args
        self.ret = ret

    @staticmethod
    def parse(line):
        name, line = pull_name(line)
        if name is None:
            return None

        assert line[0] == '('
        line = line[1:]
        args = []
        while True:
            arg, aty, line = pull_argument(line)
            if arg is None:
                break
            args.append((arg, aty))
            line = line.strip()

        ret, line = pull_return(line)

        return Function(name, args, ret)


def pull_argument(s):
    match = argument_re.match(s)
    if match is None:
        return None, None, s
    return match.group(1), match.group(2), s[match.end(3):]


def pull_name(s):
    match = name_re.match(s)
    assert match is not None
    return match.group(1), s[match.end(2):]


def pull_return(s):
    match = return_re.match(s)
    if match is None:
        return None, s
    return match.group(1), s[match.end(1):]


def read(path):
    lines = []
    with open(path) as file:
        append = False
        for line in file:
            if line == 'extern "C" {\n':
                append = True
            elif line == '}\n':
                append = False
            elif append:
                lines.append(line)
    return ''.join(lines)

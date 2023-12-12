from functools import cache

@cache
def count(s, groups):
    if len(groups) == 0:
        return 0 if '#' in s else 1
    if len(s) == 1:
        if len(groups) > 1 or groups[0] > 1:
            return 0
        if groups[0] == 1 and s[0] in ['#', '?']:
            return 1
        return 0

    res = 0
    if s[0] == '?':
        # replace with '#'
        g = list(groups[:])
        g[0] -= 1
        next_s = s[1:]
        valid = True
        if g[0] == 0:
            if next_s[0] == '#':
                valid = False
            next_s = '.' + next_s[1:]
            g.pop(0)
        else:
            if next_s[0] == '.':
                valid = False
            next_s = '#' + next_s[1:]
        if valid:
            res += count(next_s, tuple(g))

        # replace with '.'
        res += count(s[1:], groups)
    elif s[0] == '.':
        res += count(s[1:], groups)
    else:
        g = list(groups[:])
        g[0] -= 1
        next_s = s[1:]
        valid = True
        if g[0] == 0:
            if next_s[0] == '#':
                valid = False
            next_s = '.' + next_s[1:]
            g.pop(0)
        else:
            if next_s[0] == '.':
                valid = False
            next_s = '#' + next_s[1:]
        if valid:
            res += count(next_s, tuple(g))
    return res


a = [line.strip().split() for line in open('input').readlines()]
res = 0
i = 0
for s, groups in a:
    g = tuple(map(int, groups.split(',')))*5

    t = '?'.join([s]*5)
    res += count(t, g)

    i += 1
print(res)

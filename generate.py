import sys

def bfstack(text):
    res = '>\n'
    acc = 0

    for c in text:
        n = ord(c) - acc
        if abs(n) < ord(c) + 3:
            o = '+' if n > 0 else '-'
            res += o * abs(n) + '.\n'
        else:
            o = "+" * ord(c)
            res += f'[-]{o}.\n'
        acc = ord(c)

    return res

if __name__ == '__main__':
    print(bfstack(sys.argv[1] + '\n'))
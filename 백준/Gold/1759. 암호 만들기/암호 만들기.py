from sys import stdin
from itertools import permutations

L, C = map(int, stdin.readline().split())

secret = list(map(str, stdin.readline().split()))

secret.sort()


def check(string):
    mo = 0
    ja = 0
    for x in string:
        if x in 'aeiou':
            mo += 1
        else:
            ja += 1

    if mo >= 1 and ja >= 2:
        return True
    else:
        return False


def go(n, alpha, password, i):
    l = len(password)

    if l == n:
        if check(password):
            print(password)
        return

    if i == len(alpha):
        return

    go(n, alpha, password + alpha[i], i + 1)
    go(n, alpha, password, i + 1)


go(L, secret, '', 0)


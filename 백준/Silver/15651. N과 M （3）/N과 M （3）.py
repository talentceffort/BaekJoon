from sys import stdin

N, M = map(int, stdin.readline().split())

a = [0 for _ in range(10)]


def go(index, n, m):
    if index == m:
        print(*a[:m])
        return

    for i in range(1, n + 1):
        a[index] = i
        go(index + 1, n, m)


go(0, N, M)
from sys import stdin
from itertools import permutations

def next_permutation(a):
    i = len(a) - 1

    # 1. A[i - 1] < A[i] 를 만족하는 가장 큰 i 를 찾음
    while i > 0 and a[i - 1] >= a[i]:
        i -= 1

    # i 가 -1 이면 False 리턴
    if i <= 0:
        return False

    j = len(a) - 1

    # 2. j >= i 이면서 A[j] > A[i - 1] 을 만족하는 가장 큰 j 를 찾는다.
    while a[j] <= a[i - 1]:
        j -= 1

    # 3. A[i - 1] 과 A[j] 를 위치를 변경.
    a[i - 1], a[j] = a[j], a[i - 1]

    # 4. A[i] 부터 순열을 뒤집는다.
    j = len(a) - 1

    while i < j:
        a[i], a[j] = a[j], a[i]
        i += 1
        j -= 1

    return True

while True:
    n, *a = list(map(int, stdin.readline().split()))

    if n == 0:
        break

    d = [0] * (n - 6) + [1] * 6

    ans = []

    while True:
        cur = [a[i] for i in range(n) if d[i] == 1]

        ans.append(cur)
        if not next_permutation(d):
            break

    ans.sort()

    for v in ans:
        print(' '.join(map(str, v)))
    print()

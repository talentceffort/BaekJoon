# 자연수 N과 M이 주어졌을 때, 아래 조건을 만족하는 길이가 M인 수열을 모두 구하는 프로그램을 작성하시오.

# 1부터 N까지 자연수 중에서 중복 없이 M개를 고른 수열

# (1 ≤ M ≤ N ≤ 8)

from sys import stdin

N, M = map(int, stdin.readline().split())

c = [False for _ in range(10)] # 숫자의 사용 여부를 알기 위한 배열

a = [0 for _ in range(10)] # 결과를 저장 할 배열

def go(index, start, n, m):
    if index == m:
        # 수열 출력
        print(*a[:m])
        return
    for i in range(start, n + 1):
        if c[i]:
            continue
        c[i] = True
        a[index] = i
        go(index + 1, i + 1, n, m)
        c[i] = False

go(0, 1, N, M)
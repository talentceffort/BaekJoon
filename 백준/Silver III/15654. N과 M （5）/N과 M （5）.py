from sys import stdin, stdout

N, M = map(int, stdin.readline().split())

num = list(map(int, stdin.readline().split()))

a = [0 for _ in range(10)] # 결과를 저장 할 배열

c = [False for _ in range(10)] # 숫자의 사용 여부를 알기 위한 배열

def go(index, start, n, m):
    if index == m:
        # 수열 출력
        print(*a[:m])
        return
    for i in range(start, n + 1):
        if c[i]:
            continue
        a[index] = num[i - 1]
        c[i] = True
        go(index + 1, 1, n, m)
        c[i] = False

num.sort()
go(0, 1, N, M)
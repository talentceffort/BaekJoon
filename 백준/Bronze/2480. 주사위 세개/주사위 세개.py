import sys

N, M, L = map(int, sys.stdin.readline().split())

# 모두 같은 경우
if N == M == L:
    print(10000 + (N * 1000))
# 모두 다른 경우
elif N != M and M != L and N != L:
    max_num = max(N, M, L)
    print(max_num * 100)
# 두 수만 같은 경우
else:
    # 두 수가 같은 경우, 그 수를 찾아서 계산
    if N == M or N == L:
        print(1000 + N * 100)
    else:  # M == L
        print(1000 + M * 100)

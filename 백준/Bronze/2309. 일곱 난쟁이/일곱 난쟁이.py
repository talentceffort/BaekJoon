from sys import stdin

nan = []

for _ in range(9):
    nan.append(int(stdin.readline()))

sum_tall = sum(nan)

nan.sort()

for i in range(9):
    for j in range(9):
        if sum_tall - nan[i] - nan[j] == 100:
            for k in range(9):
                if i == k or j == k:
                    continue
                else:
                    print(nan[k])
            exit()
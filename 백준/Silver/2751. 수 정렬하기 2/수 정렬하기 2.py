import sys
input=sys.stdin.readline

n = int(input())
array = []

for i in range(n):
    array.append(int(input()))

for i in sorted(array):
    print(i)
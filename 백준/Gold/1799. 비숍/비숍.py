import sys

input = sys.stdin.readline


def place_bishops(color, index, count, placed1, placed2):
    if index == len(diagonals[color]):
        return count
    max_count = count
    for x, y in diagonals[color][index]:
        if not placed1[x + y] and not placed2[x - y + N - 1]:
            placed1[x + y] = placed2[x - y + N - 1] = True
            max_count = max(
                max_count, place_bishops(color, index + 1, count + 1, placed1, placed2)
            )
            placed1[x + y] = placed2[x - y + N - 1] = False
    max_count = max(max_count, place_bishops(color, index + 1, count, placed1, placed2))
    return max_count


N = int(input())
board = [list(map(int, input().split())) for _ in range(N)]

# 대각선 방향에 따라 비숍을 놓을 수 있는 위치를 저장 (흑, 백)
diagonals = [[[] for _ in range(2 * N - 1)] for _ in range(2)]

for x in range(N):
    for y in range(N):
        if board[x][y] == 1:
            diagonals[(x + y) % 2][x + y].append((x, y))

# 각 색깔별로 비숍을 최대한 배치
max_bishops = 0
for color in range(2):
    placed1 = [False] * (2 * N - 1)
    placed2 = [False] * (2 * N - 1)
    max_bishops += place_bishops(color, 0, 0, placed1, placed2)

print(max_bishops)

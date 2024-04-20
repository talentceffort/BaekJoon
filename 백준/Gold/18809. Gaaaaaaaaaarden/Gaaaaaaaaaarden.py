import sys
from collections import deque
from itertools import combinations

input = sys.stdin.readline

# 방향 벡터를 좀 더 읽기 쉽게 튜플의 리스트로 정의
directions = [(1, 0), (-1, 0), (0, 1), (0, -1)]


def bfs():
    flower = 0
    while dq:
        y, x, y_last, x_last, time, color = dq.popleft()
        if visited[y_last][x_last] == 1:  # 이전에 꽃이 피었으면 더 이상 진행하지 않음
            continue
        if visited[y][x]:
            if visited[y][x] == (
                time,
                -color,
            ):  # 서로 다른 색상의 배양액이 만나 꽃이 핌
                visited[y][x] = 1
                flower += 1
            continue
        visited[y][x] = (time, color)
        for dy, dx in directions:
            ny, nx = y + dy, x + dx
            if (
                0 <= ny < N and 0 <= nx < M and garden[ny][nx]
            ):  # 유효한 범위이고, 배양 가능한 땅인 경우
                dq.append((ny, nx, y, x, time + 1, color))
    return flower


N, M, G, R = map(int, input().split())
garden = [list(map(int, input().split())) for _ in range(N)]
spread = [(i, j) for i in range(N) for j in range(M) if garden[i][j] == 2]


result = 0
for g_and_r_positions in combinations(spread, G + R):
    for g_positions in combinations(g_and_r_positions, G):
        visited = [[0] * M for _ in range(N)]
        dq = deque()
        # G 배양액과 R 배양액의 초기 위치 설정
        for y, x in g_and_r_positions:
            color = 1 if (y, x) in g_positions else -1
            dq.append((y, x, y, x, 1, color))
        result = max(result, bfs())

print(result)

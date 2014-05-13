#include <iostream>
#include <queue>
#include <utility>
#include <vector>
using namespace std;

typedef vector<vector<char>> VVC;
typedef vector<vector<int>> VVI;
typedef pair<int,int> P;

bool bfs(VVC& m, VVI& d, queue<P>& que);

const int INF = 100000000;
const int dx[] = {0, 1, 0, -1}, dy[] = {1, 0, -1, 0};
int X = 0, Y = 0;

int main() {
  cin >> Y >> X;

  int sx, sy, gx, gy;

  VVC maze(Y, vector<char>(X));
  VVI d(Y, vector<int>(X));

  for (int y = 0; y < Y; ++y) {
    for (int x = 0; x < X; ++x) {
      cin >> maze[y][x];
      d[y][x] = INF;
      if (maze[y][x] == 'S') {
        sx = x; sy = y;
      } else if (maze[y][x] == 'G') {
        gx = x; gy = y;
      }
    }
  }
  queue<P> que;
  que.emplace(sx,sy);
  d[sy][sx] = 0;
  while (!que.empty() && !bfs(maze,d,que));
  cout << d[gy][gx] << endl;
}

bool bfs(VVC& m, VVI& d, queue<P>& que) {
  const P p = que.front();
  que.pop();

  const int x = p.first, y = p.second;
  
  if (m[y][x] == 'G') return true;
  
  for (int i = 0; i < 4; ++i) {
    const int nx = x + dx[i], ny = y + dy[i];
    if (nx >= 0 && ny >= 0 && nx < X && ny < Y && m[ny][nx] != '#' && d[ny][nx] == INF) {
      que.emplace(nx,ny);
      d[ny][nx] = d[y][x] + 1;
    }
  }
  return false;
}

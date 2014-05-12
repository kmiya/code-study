#include <iostream>
#include <vector>
using namespace std;

void dfs(int h, int w, vector<vector<char>>& f);

int H = 0;
int W = 0;

int main() {
  cin >> H >> W;
  vector<vector<char>> f(H, vector<char>(W));
  for (auto h = 0; h < H; ++h) {
    for (auto w = 0; w < W; ++w) {
      cin >> f[h][w];
      cout << f[h][w];
    }
    cout << endl;
  }
  int ans = 0;
  for (auto h = 0; h < H; ++h) {
    for (auto w = 0; w < W; ++w) {
      if (f[h][w] == 'W') {
        ++ans;
        dfs(h,w,f);
      }
    }
  }
  cout << ans << endl;
}

void dfs(int h, int w, vector<vector<char>>& f) {
  f[h][w] = '.';
  for (auto dh = h - 1; dh < h + 2; ++dh) {
    for (auto dw = w - 1; dw < w + 2; ++dw) {
      if (dh >= 0  && dw >= 0 && dh < H && dw < W
          && f[dh][dw] == 'W')
        dfs(dh, dw, f);
    }
  }
}

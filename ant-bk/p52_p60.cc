#include <iostream>
#include <limits>
#include <numeric>
#include <vector>
using namespace std;

typedef long long ll;

int main() {
  constexpr ll INF = numeric_limits<ll>::max();
  int TEST;
  cin >> TEST;
  for (int test = 0; test < TEST; ++test) {

    int n;
    ll W;
    cin >> W >> n;
    vector<int> v;
    vector<ll> w;
    for (int i = 0; i < n; ++i) {
      int tv, tw;
      cin >> tw >> tv;
      v.push_back(tv);
      w.push_back(tw);
    }

    const int V_SUM = accumulate(v.begin(), v.end(), 0);
    vector<vector<ll>> dp(V_SUM + 1, vector<ll>(n, INF));
    for (int y = 0; y <= V_SUM; ++y)
      if (y <= v[0]) dp[y][0] = w[0];

    int acc = v[0];
    for (int x = 1; x < n; ++x) {
      for (int y = 0; y <= v[x] + acc; ++y) {
        dp[y][x] = y <= v[x]
                 ? min(dp[y][x - 1], w[x])
                 : min(dp[y][x - 1], w[x] + dp[y - v[x]][x - 1]);
      }
      acc += v[x];
    }

    int ans = 0;
    acc = 0;
    for (int x = 0; x < n; ++x) {
      for (int y = 0; y <= v[x] + acc; ++y) {
        if (dp[y][x] <= W) ans = max(ans, y);
      }
      acc += v[x];
    }
    cout << "Case #" << test << ": " << ans << endl;

    // debug
    // for (int x = 0; x < n; ++x) {
    //   for (int y = 0; y <= V_SUM; ++y) {
    //     if (dp[y][x] == INF) cout << "I ";
    //     else if (dp[y][x] < 0) cout << "- ";
    //     else cout << dp[y][x] << " ";
    //   }
    //   cout << endl;
    // }
  }
}

#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

int main() {
  int TEST;
  cin >> TEST;
  for (int test = 0; test < TEST; ++test) {
    int R, n;
    cin >> R >> n;
    vector<int> x;
    for (int i = 0; i < n; ++i) {
      int army;
      cin >> army;
      x.push_back(army);
    }
    sort(x.begin(), x.end());

    int ans = 0, h = 0, t = 0;
    for (; h < n; h = t) {
      while (t < n && x[t] <= x[h] + R) ++t;
      h = t - 1;
      while (t < n && x[t] <= x[h] + R) ++t;
      ++ans;
    }
    cout << ans << endl;
  }
}

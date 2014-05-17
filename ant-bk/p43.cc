#include <algorithm>
#include <iostream>
#include <utility>
#include <vector>
using namespace std;

int main() {
  int T;
  cin >> T;
  for (int t = 0; t < T; ++t) {
    int n;
    cin >> n;
    vector<pair<int, int>> task;
    for (int i = 0; i < n; ++i) {
      int ts, tt;
      cin >> ts >> tt;
      task.emplace_back(tt,ts);
    }
    sort(task.begin(),task.end()); // 終了時間を昇順sort
    int ans = 0, end_time = 0;
    for (int i = 0; i < n; ++i) {
      if (task[i].second > end_time) {
        ++ans;
        end_time = task[i].first;
      }
    }
    cout << ans << endl;
  }
}

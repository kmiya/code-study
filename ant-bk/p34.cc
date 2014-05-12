#include <iostream>
#include <vector>
using namespace std;

bool dfs(int i, int sum, vector<int>& a);

int K = 0;
int N = 0;

int main() {
  int T = 0;
  cin >> T;
  for (auto t = 0; t < T; ++t) {
    cin >> K >> N;
    vector<int> a;
    for (auto n = 0; n < N; ++n) {
      int tmp;
      cin >> tmp;
      a.push_back(tmp);
      // cout << a[n] << " ";
    }
    if (dfs(0,0,a)) cout << "Yes" << endl;
    else cout << "No" << endl;
  }  
}

bool dfs(int i, int sum, vector<int>& a) {
  if (sum > K) return false;
  
  if (i == N) return K == sum;

  if (dfs(i+1, sum, a)) return true;

  if (dfs(i+1, sum + a[i], a)) return true;

  return false;
}

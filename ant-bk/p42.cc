#include <iostream>
using namespace std;

int main() {
  const int V[6] = {1, 5, 10, 50, 100, 500};
  int coin[6];
  for (int i = 0; i < 6; ++i) {
    cin >> coin[i];
  }
  int A, ans = 0;
  cin >> A;
  for (int i = 5; i >= 0; --i) {
    const int n = min(coin[i], A / V[i]);
    A -= n * V[i];
    ans += n;
  }
  cout << ans << endl;
}

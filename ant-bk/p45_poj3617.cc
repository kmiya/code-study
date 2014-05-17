#include <iostream>
#include <string>
using namespace std;

int main() {
  int TEST;
  cin >> TEST;
  for (int test = 0; test < TEST; ++test) {
    int N;
    string S;
    cin >> N >> S;

    int h = 0, t = N - 1;
    while (h <= t) {
      bool select_left = false;
      for (int i = 0; h + i < t; ++i) {
        if (S[h + i] < S[t - i]) {
          select_left = true;
          break;
        } else if (S[h + i] > S[t - i]) {
          break;
        }
      }

      if (select_left) cout << S[h++];
      else cout << S[t--];
    }
    cout << endl;
  }
}

#include <algorithm>
#include <iostream>
#include <vector>
using namespace std;

int main() {
  int TEST;
  cin >> TEST;
  for (int test = 0; test < TEST; ++test) {
    int N;
    vector<long long> L;
    cin >> N;
    for (int i = 0; i < N; ++i) {
      int l = 0;
      cin >> l;
      L.push_back(l);
    }

    // O(n(n+1)/2) = O(n^2) solution
    int fst = 0, snd = 1;
    long long ans = 0;
    while (snd < N) {
      if (L[fst] > L[snd]) swap(L[fst], L[snd]);
      for (int i = snd + 1; i < N; ++i) {
        if (L[fst] > L[i]) swap(L[fst],L[i]);
        if (L[snd] > L[i]) swap(L[snd],L[i]);
      }
      L[snd] += L[fst++];
      ans += L[snd++];
    }
    cout << ans << endl;
  }
}

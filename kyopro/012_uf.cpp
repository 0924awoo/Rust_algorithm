#include <bits/stdc++.h>
using namespace std;
#define ll int64_t

struct UnionFind {
  vector<ll> parents;
  UnionFind(ll n) {
    // -1:　白,  0以上: 赤
    parents = vector<ll>(n, -1);
  }
  ll find(ll x) {
    if (parents.at(x) < 0) {
      return x;
    } else {
      parents[x] = find(parents[x]);
      return parents[x];
    }
  }
  void unite(ll x, ll y) {
    x = find(x);
    y = find(y);
    // already united
    if (x == y)
      return;
    if (parents[x] > parents[y])
      swap(x, y);
    parents[x] += parents[y];
    parents[y] = x;
  }
  ll size(ll x) {
    return -parents[find(x)];
  }
  bool issame(ll x, ll y) {
    return find(x) == find(y);
  }
};


int main() {
 
  ll H, W, Q;
  cin >> H >> W >> Q;
  
  // H * W
  UnionFind un(H*W);
  int t, ra, ca, rb, cb;
  
  vector<bool> is_red;
  for(int i=0; i < H*W; i++) {
   is_red.push_back(false);
  }
  
  for(int i=0; i<Q; i++) {
    cin >> t >> ra >> ca;
    ra--;
    ca--;
    if (t == 1) {
      is_red.at(W*ra + ca) = true;
      // 上
      if (ra != 0) {       
        if (is_red.at(W*(ra-1) + ca)) {
          un.unite(W*(ra-1) + ca, W*ra + ca);

        }    
      }
      
      // 下 
      if (ra != H-1) {
        if (is_red.at(W*(ra+1) + ca)) {
          un.unite(W*(ra+1) + ca, W*ra + ca);
        }        
      }
      
      // 左
      if (ca != 0) {
        if (is_red.at(W*ra + (ca-1))) {
          un.unite(W*ra + (ca-1), W*ra + ca);
        }        
      }
      
      // 右
      if (ca != W-1) {
        if (is_red.at(W*ra + (ca+1))) {
          un.unite(W*ra + (ca+1), W*ra + ca);
        }            
      }
      //un.parents.at(W*ra + ca) -= 1;
      //cout << un.parents[W*ra + ca] << endl;
        
    } else {
      // t==2
      cin >> rb >> cb;
      rb--;
      cb--;
      if (!(is_red.at(W*ra + ca)) || !(is_red.at(W*rb + cb))) {
        cout << "No" << endl;
        continue;
      }
      if (un.issame(W*ra + ca, W*rb + cb)) {
        cout << "Yes" << endl;
      } else {
        cout << "No" << endl;
      }
      
    }
  }
}
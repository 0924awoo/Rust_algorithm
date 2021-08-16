struct UnionFind {
 parents: Vec<i64>,
}

impl UnionFind {
 fn new(n: i64) -> Self {
   let mut p: Vec<i64> = Vec::new();
   for i in 0..n {
     p.push(-1);
   }
   UnionFind {parents: p}
 }
 
 fn find(&mut self, x: i64) -> i64 {
   if self.parents[x as usize] < 0 {
     return x;
   } else {
     self.parents[x as usize] = self.find(self.parents[x as usize]);
     return self.parents[x as usize];
   }
 }

 fn unite(&mut self, x: i64, y: i64) {
	let mut px = self.find(x);
   let mut py = self.find(y);
   
   // already united
   if x == y {
     return;
   }
   
   // i.e. parents[py]:-4, parents[px]:-2
   if self.parents[py as usize] < self.parents[px as usize] {
     let tmp = py;
     py = px;
     px = tmp;
   }
   self.parents[px as usize] += self.parents[py as usize];
   self.parents[py as usize] = x;
 }
 
 fn size(&mut self, x: i64) -> i64 {
   let ans = self.find(x);
   return -self.parents[ans as usize];
 }
 
 fn issame(&mut self, x: i64, y: i64) -> bool {
   return self.find(x) == self.find(y);
 }
 
}
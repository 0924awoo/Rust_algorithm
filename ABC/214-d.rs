struct PriorityQueue {
    tree: Vec<(usize, usize)>,
    size: usize,
}

impl PriorityQueue {
    fn new() -> Self {
        return PriorityQueue {tree: Vec::new(), size: 0};
    }

    fn add(&mut self, value: usize, index: usize) {
        // add to bottom
        self.tree.push((value, index));
        self.size += 1;

        // modify
        let mut now_index = self.size - 1;
        let mut parent_index = (now_index - 1) / 2;
        
        let mut tmp_node: (usize, usize) = (0, 0);
        while now_index != 0 {
            if self.tree[now_index].0 >= self.tree[parent_index].0 {
                break;
            }
          //println!("{} {} {}", self.tree.len(), now_index, x);
            tmp_node= self.tree[parent_index];
            self.tree[parent_index] = self.tree[now_index];
            self.tree[now_index] = tmp_node;

            now_index = parent_index;
            parent_index = (parent_index - 1) / 2;
        }
        
    }

    fn head(&self) -> (usize, usize) {
        self.tree[0]
    }

    // remove head and move tail to head
    fn pop(&mut self) {
        self.size -= 1;

      	//println!("{}", self.tree.len()-1);
        if self.size == 0 {
          self.tree.remove(0);
          return;
        } else {
            self.tree[0] = self.tree.remove(self.size);
        }

		//println!("{}", ans);
        // modify
        let mut now_index = 0;
        let mut next_index = 0;
        let last_index = self.size - 1;
        let mut l = 0;
        let mut r = 0;
        let mut tmp_node = (0,0);

        loop {
            l = now_index*2 + 1;
            r = now_index*2 + 2;
            if l > last_index {
                break;
            } else if l == last_index {
                if self.tree[now_index].0 > self.tree[l].0 {
                    tmp_node = self.tree[l];
                    self.tree[l] = self.tree[now_index];
                    self.tree[now_index] = tmp_node;
                }
                break;
            } else {
                if self.tree[l].0 < self.tree[r].0 {
                    next_index = l;
                } else {
                    next_index = r;
                }
                if self.tree[next_index].0 < self.tree[now_index].0 {
                    tmp_node = self.tree[next_index];
                    self.tree[next_index] = self.tree[now_index];
                    self.tree[now_index] = tmp_node;
                    now_index = next_index;
                } else {
                    break;
                }
            }
        }

    }

    fn size(&self) -> usize {
        self.size
    }
}

struct UnionFind {
    parents: Vec<i32>,
   }
   
   impl UnionFind {
    fn new(n: usize) -> Self {
      let mut p: Vec<i32> = vec![-1; n];
      UnionFind {parents: p}
    }
    
    fn find(&mut self, x: i32) -> i32 {
      if self.parents[x as usize] < 0 {
        return x;
      } else {
        self.parents[x as usize] = self.find(self.parents[x as usize]);
        return self.parents[x as usize];
      }
    }
   
    fn unite(&mut self, x: i32, y: i32) {
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
    
    fn size(&mut self, x: i32) -> i32 {
      let ans = self.find(x);
      return -self.parents[ans as usize];
    }
    
    fn issame(&mut self, x: i32, y: i32) -> bool {
      return self.find(x) == self.find(y);
    }
    
}

fn main(){

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut n: usize = s.trim().parse().unwrap();

    // set roads (weight, u, v)
    let mut pq = PriorityQueue::new();
    let mut roads: Vec<(usize, usize, usize)> = Vec::new();
    for i in 0..n-1 {
        s.clear();
        std::io::stdin().read_line(&mut s).ok();
        let mut spl = s.trim().split(' ');
        let mut u: usize = spl.next().unwrap().parse().unwrap();
        let mut v: usize = spl.next().unwrap().parse().unwrap();
        let mut w: usize = spl.next().unwrap().parse().unwrap(); 
        roads.push((w, u-1, v-1));
        pq.add(w, i);
    }

    let mut uf = UnionFind::new(n);
    let mut count: i64 = 0;

    while pq.size != 0 {
        // r: (weight, index)
        let r = pq.head();
        pq.pop();
        let index = r.1;

        let w = roads[index].0;
        let u = roads[index].1;
        let v = roads[index].2;
      
        count += ((uf.size(u as i32) * uf.size(v as i32)) * (w as i32)) as i64;
      
        uf.unite(u as i32, v as i32);
      
    }

    println!("{}", count);
} 
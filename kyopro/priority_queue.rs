struct PriorityQueue {
    tree: Vec<u32>,
}

impl PriorityQueue {
    fn new(x: u32) -> Self {
//        for i in 0..n {
//            tmp.push(Node {value: 0, parents: -1, left: -1, right: -1});
//        }
        let mut init: Vec<u32> = Vec::new();
        init.push(x);

        return PriorityQueue {tree: init};
    }

    fn add(&mut self, x: u32) {
        // add to bottom
        self.tree.push(x);

        // modify
        let mut parent_index = (self.tree.len()-1) / 2;
        let mut now_index = self.tree.len() - 1;
        let mut tmp_value = 0;
        while now_index != 0 {
            if self.tree[now_index] >= self.tree[parent_index] {
                break;
            }
          //println!("{} {} {}", self.tree.len(), now_index, x);
            tmp_value = self.tree[parent_index];
            self.tree[parent_index] = self.tree[now_index];
            self.tree[now_index] = tmp_value;

            now_index = parent_index;
            parent_index = (parent_index - 1) / 2;
        }
        
    }

    fn head(&mut self) -> u32 {
        // remove head and move tail to head
        let ans = self.tree[0];
      	//println!("{}", self.tree.len()-1);
        if (self.tree.len()-1) == 0 {
          self.tree.remove(0);
          return ans;
      } else {
        self.tree[0] = self.tree.remove(self.tree.len() - 1);
      }
		//println!("{}", ans);
        // modify
        let mut now_index = 0;
        let mut next_index = 0;
        let last_index = self.tree.len() - 1;
        let mut l = 0;
        let mut r = 0;
        let mut tmp = 0;

        loop {
            l = now_index*2 + 1;
            r = now_index*2 + 2;
            if l > last_index {
                break;
            } else if l == last_index {
                if self.tree[now_index] > self.tree[l] {
                    tmp = self.tree[l];
                    self.tree[l] = self.tree[now_index];
                    self.tree[now_index] = tmp;
                }
                break;
            } else {
                if self.tree[l] < self.tree[r] {
                    next_index = l;
                } else {
                    next_index = r;
                }
                if self.tree[next_index] < self.tree[now_index] {
                    tmp = self.tree[next_index];
                    self.tree[next_index] = self.tree[now_index];
                    self.tree[now_index] = tmp;
                    now_index = next_index;
                } else {
                    break;
                }
            }
        }

        return ans;
    }
}


fn main() {

  let mut hs = PriorityQueue::new(11);

  hs.add(4);
  hs.add(7);
  hs.add(9);
  hs.add(4);
  hs.add(7);
  hs.add(2);
  
  for i in 0..7{
      println!("{}", hs.head());
  }
  hs.add(13);
  hs.add(10);
  hs.add(12);
  for i in 0..3{
    println!("{}", hs.head());
}
}
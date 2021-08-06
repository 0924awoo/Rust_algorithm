struct PriorityQueue {
    tree: Vec<(usize, usize)>,
    size: usize,
}

impl PriorityQueue {
    fn new(value: usize, index: usize) -> Self {

        let mut init: Vec<(usize, usize)> = Vec::new();
        init.push((value, index));

        return PriorityQueue {tree: init, size: 1};
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

    fn head(&mut self) -> (usize, usize) {
        // remove head and move tail to head
        let ans = self.tree[0];
        self.size -= 1;

      	//println!("{}", self.tree.len()-1);
        if self.size == 0 {
          self.tree.remove(0);
          return ans;
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

        return ans;
    }

    fn size(&self) -> usize {
        self.size
    }
}


fn Dijkstra(v_num: usize, edges: Vec<usize, (usize, usize)>, start: usize) -> Vec<usize> {
    let INF = 1000000000 + 9;
    
    // setting pq and d
    let mut pq = PriorityQueue::new(0,start);
    let mut d = Vec::new();
    let mut prev = Vec::new();
    for i in 0..v_num{
        if start != i {
            pq.add(inf, i);
            d.push(inf);
        } else {
            d.push(0);
        }
        prev.push(-1);
    }

    // setting e
    let mut e = Vec::new();
    for i in 0..edges.len() {
        e.push(edges[i]);
    }

    
    while pq.size != 0 {
        // n.0: now_city's weight, n.1: now_city
        let n = pg.head();
        let d_now= n.0;
        let now_city = n.1;
        for i in 0..e[now_city].len() {
            // road.0: destination, road.1: weight
            let road = e[now_city][i];
            let alt = d_now + road.1;
            if alt < d[road.0] {
                d[road.0] = alt;
                prev[road.0] = now_city;
                pg.add(alt, road.0);
            }
        }
    }

    return d;
} 


fn main() {

  let mut hs = PriorityQueue::new(11, 0);
  hs.add(13, 1);
  hs.add(10, 2);
  hs.add(12, 3);
  hs.add(4, 4);
  hs.add(7, 5);
  hs.add(9, 6);
  hs.add(4, 7);
  hs.add(7, 8);
  hs.add(2, 9);
  
  let mut a = (0, 0);
  for i in 0..10{
    a = hs.head();
      println!("{} {}", a.0, a.1);
  }
}
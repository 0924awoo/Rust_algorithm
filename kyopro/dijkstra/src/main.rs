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


fn main(){
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut spl = s.trim().split(' ');
    let mut n: usize = spl.next().unwrap().parse().unwrap();
    let mut m: usize = spl.next().unwrap().parse().unwrap();

    let mut e: Vec<Vec<(usize, usize)>> = Vec::new();
    
    // set edges
    for i in 0..m {
        e.push(Vec::new());
    }
    
    for i in 0..m {
        s.clear();
        std::io::stdin().read_line(&mut s).ok();
        let mut spl = s.trim().split(' ');
        let mut a: usize = spl.next().unwrap().parse().unwrap();
        let mut b: usize = spl.next().unwrap().parse().unwrap();
        let mut c: usize = spl.next().unwrap().parse().unwrap();        
        e[a-1].push((b-1, c));
    }
    // d1: (weight, prev city)
    let d1: Vec<(usize, i32)> = dijkstra(n, &e, 0);

    for i in 0..n {
        println!("result: {}, {}, prev:{}", i, d1[i].0, d1[i].1);
    }

} 

fn dijkstra(v_num: usize, edges: &Vec<Vec<(usize, usize)>>, start: usize) -> Vec<(usize, i32)> {
    let INF = 1000000000 + 9;

    let n = v_num;
    let m = edges.len();
    let mut e: Vec<Vec<(usize, usize)>> = Vec::new();
    
  	println!("{} {} {}", n, m, start);
    // set pq and d adn edges
    let mut pq = PriorityQueue::new(0,start);
    // d[v] : (min weight, prev city)
    let mut d: Vec<(usize, i32)> = Vec::new();
    for i in 0..n{
        if start != i {
            pq.add(INF, i);
            d.push((INF, -1));
        } else {
            d.push((0, -1));
        }
        e.push(edges[i]);
    }
    
    // 
    while pq.size != 0 {
        // now.0: now_city's weight, now.1: now_city
        let now = pq.head();
        pq.pop();
        let d_now= now.0;
        let now_city = now.1;
        //println!("{} {}", now_city, d_now);
        if d[now_city].0 < d_now {
            continue;
        }

        for i in 0..e[now_city].len() {
            // road.0: destination, road.1: weight
            //println!("roads:{}", i);
            let road = e[now_city][i];
            let alt = d_now + road.1;
            if alt < d[road.0].0 {
                d[road.0] = (alt, now_city as i32);
                pq.add(alt, road.0);
            }
        }
    }
    for i in 0..n {
        println!("fn result: {}, {}, prev:{}", i, d[i].0, d[i].1);
    }
    return d;
}
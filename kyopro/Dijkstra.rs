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


fn Dijkstra(v_num: usize, edges: &Vec<Vec<(usize, usize)>>, start: usize, d: &mut Vec<(usize, i32)>) {
    // v_num: a number of cities    
    // edges: edges[city][road's index] = (next city, road's weight)
    // i.e. edges[city][0] = (3, 6)
    // start: start city's index
    // d: d[distination city] = (min weight, prev city)
 
    let inf = 1000000000 + 9;
    
    // setting pq
    let mut pq = PriorityQueue::new(0,start);
    for i in 0..v_num{
        if start != i {
            pq.add(inf, i);
            d.push((inf, -1));
        } else {
            d.push((0, -1));
        }
    }


    while pq.size != 0 {
        // n.0: now_city's weight, n.1: now_city
        let n = pq.head();
        let d_now= n.0;
        let now_city = n.1;
        for i in 0..edges[now_city].len() {
            // road.0: destination, road.1: weight
            let road = edges[now_city][i];
            let alt = d_now + road.1;
            if alt < d[road.0].0 {
                d[road.0] = (alt, now_city as i32);
                pq.add(alt, road.0);
            }
        }
    }

} 


fn main() {

    let inf = 1000000000 + 9;

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut spl = s.trim().split(' ');
    let mut n: usize = spl.next().unwrap().parse().unwrap();
    let mut m: usize = spl.next().unwrap().parse().unwrap();

    // set edges 
    // i.e. e[city][0] = (3, 6) 
    //  (next city, road's weight)
    let mut e: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n];
    for i in 0..m {
        s.clear();
        std::io::stdin().read_line(&mut s).ok();
        let mut spl = s.trim().split(' ');
        let mut a: usize = spl.next().unwrap().parse().unwrap();
        let mut b: usize = spl.next().unwrap().parse().unwrap();
        let mut c: usize = spl.next().unwrap().parse().unwrap();        
        e[a-1].push((b-1, c));
        e[b-1].push((a-1, c));
    }

    let mut d0: Vec<(usize, i32)> = vec![(inf, -1); n];
    let mut dn: Vec<(usize, i32)> = vec![(inf, -1); n];

    // d0
    Dijkstra(n, &e, 0, &mut d0);

    // dn
    Dijkstra(n, &e, n-1, &mut dn);

    println!("{}", dn[0].0);

    for i in 1..n-1 {
        println!("{}", d0[i].0 + dn[i].0);
    }

    // city n-1
    println!("{}", d0[n-1].0);

}
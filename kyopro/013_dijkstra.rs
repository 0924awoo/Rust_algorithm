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

    let INF = 1000000000 + 9;
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut spl = s.trim().split(' ');
    let mut n: usize = spl.next().unwrap().parse().unwrap();
    let mut m: usize = spl.next().unwrap().parse().unwrap();

    // set edges
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

    // 0: white, 1: grey, 2: black
    let mut citys: Vec<u8> = vec![0; n];
    
    // set pq and d
    let mut pq = PriorityQueue::new(0,0);
    // d[v] : (min weight, prev city)
    let mut d: Vec<(usize, i32)> = vec![(INF, -1); n];
    d[0] = (0, -1);
    for i in 1..n{
        pq.add(INF, i);
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
            if citys[road.0] == 2 {
                continue;
            }
            let alt = d_now + road.1;
            if alt < d[road.0].0 {
                d[road.0] = (alt, now_city as i32);
                pq.add(alt, road.0);
            }
        }
        citys[now_city] = 2;
    }


    // n-1
    let start = n-1;
    // set pq and d
    let mut pq = PriorityQueue::new(0,n-1);
    // d[v] : (min weight, prev city)
    let mut dk: Vec<(usize, i32)> = vec![(INF, -1); n];
    dk[n-1] = (0, -1);
    for j in 1..n{
        pq.add(INF, j);
    }
    // 0: white, 1: grey, 2: black
    let mut citys: Vec<u8> = vec![0; n];
    
    // 
    while pq.size != 0 {
        // now.0: now_city's weight, now.1: now_city
        let now = pq.head();
        pq.pop();
        let d_now= now.0;
        let now_city = now.1;
        //println!("{} {}", now_city, d_now);
        if dk[now_city].0 < d_now {
            continue;
        }

        for i in 0..e[now_city].len() {
            // road.0: destination, road.1: weight
            //println!("roads:{}", i);
            let road = e[now_city][i];
            if citys[road.0] == 2 {
                continue;
            }
            let alt = d_now + road.1;
            if alt < dk[road.0].0 {
                dk[road.0] = (alt, now_city as i32);
                pq.add(alt, road.0);
            }
        }
    }

    // city 0
    println!("{}", d[n-1].0);

    for i in 1..n-1 {
        println!("{}", d[start].0 + dk[start].0);
    }

    // city n-1
    println!("{}", dk[0].0);

} 
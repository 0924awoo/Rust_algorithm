fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let n: usize = s.trim().parse().unwrap();
  
  s.clear();
  
  let mut roads: Vec<Vec<usize>> = Vec::new();
  
  for i in 0..n {
    roads.push(Vec::new());
  }
  
  for i in 0..n-1 {
    s.clear();
    std::io::stdin().read_line(&mut s).ok();
    let mut splt = s.trim().split(' ');
    let mut left: usize = splt.next().unwrap().parse().unwrap();
    let mut right: usize = splt.next().unwrap().parse().unwrap();
    left -= 1;
    right -= 1;
    roads[left].push(right);
    roads[right].push(left);
  }
  
  let dist_first = bfs(&roads, 0, n);
  let mut max1 = 0;
  let mut max_town = 0;
  for i in 0..n {
    if max1 < dist_first[i] {
      max1 = dist_first[i];
      max_town = i;
    }
  }
 
  let dist_ans = bfs(&roads, max_town, n);
  let mut max2 = 0;
  for i in 0..n {
    if max2 < dist_ans[i] {
      max2 = dist_ans[i];
    }
  }
  
  println!("{}", max2+1);
 
}

fn bfs(roads: &Vec<Vec<usize>>, start: usize, n: usize) -> Vec<isize> {
  let mut dist = Vec::new();
  for i in 0..n {
    dist.push(-1);
  }
  let mut queue = Vec::new();
  queue.push(start);
  let mut pos = start;
  dist[start] = 0;
  
  while queue.len() != 0 {
    pos = queue.remove(0);
    for i in 0..roads[pos].len() {
      let to = roads[pos][i];
      if dist[to] == -1 {
      	dist[to] = dist[pos] + 1;
        queue.push(to);
      }
  	}
  }
  
  return dist;
}
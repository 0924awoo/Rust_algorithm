fn main() {
  
  let mut an: Vec<usize> = Vec::new();
  let mut bn: Vec<usize> = Vec::new();
  
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut n: usize = s.trim().parse().unwrap();
  
  s.clear();
  std::io::stdin().read_line(&mut s).ok();
  let mut splt = s.trim().split(' ');
  for i in 0..n {
    let mut a: usize = splt.next().unwrap().parse().unwrap();
    an.push(a);
  }
  
  s.clear();
  std::io::stdin().read_line(&mut s).ok();
  let mut splt = s.trim().split(' ');
  for i in 0..n {
    let mut b: usize = splt.next().unwrap().parse().unwrap();
    bn.push(b);
  }
  
  an.sort();
  bn.sort();
  
  let mut ans = 0;
  for i in 0..n {
    ans += sub(an[i], bn[i]);
  }
  println!("{}", ans);
}

fn sub(x: usize, y: usize) -> usize {
  if x < y {
    return y - x;
  } else {
    return x - y;
  }
}
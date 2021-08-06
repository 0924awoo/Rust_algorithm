fn main() {
  let modu = 1000000000 + 7;
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let mut spl = s.trim().split(' ');
  let mut n: usize = spl.next().unwrap().parse().unwrap();
  let mut k: usize = spl.next().unwrap().parse().unwrap();

  s.clear();
  std::io::stdin().read_line(&mut s).ok();
  let init_str = s.trim();
  
  // pos: 0 ~ n-1
  let mut pos = 0;
  let mut str_c: Vec<char> = init_str.chars().collect();
  let mut ans : Vec<char> = Vec::new();
  
  while k > 0 {
    let mut small_c = 'z';
    let mut small_pos = pos;
    for j in pos..n-k+1 {
      if str_c[j] < small_c {
        small_c = str_c[j];
        small_pos = j;
      }
    }
    //println!("{}", n-k);
    ans.push(small_c);
    pos = small_pos + 1;
    
    k -= 1;
  }
  
  let a: String = ans.iter().collect();
  println!("{}", &a);
  
}
 
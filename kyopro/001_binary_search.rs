fn main() {
  let mut s = String::new();
  std::io::stdin().read_line(&mut s).ok();
  let mut splt = s.trim().split(' ');
  let N: u32= splt.next().unwrap().parse().unwrap();
  let L: u64= splt.next().unwrap().parse().unwrap();
  
  s.clear();
  std::io::stdin().read_line(&mut s).ok();
  let K: u32 = s.trim().parse().unwrap();
  
  s.clear();
  std::io::stdin().read_line(&mut s).ok();
  let mut splt = s.trim().split(' ');
  let mut AN: Vec<u64> = Vec::new();
  
  for an in splt {
    let mut tmp: u64 = an.parse().unwrap();
    AN.push(tmp);
  }
  
  let mut tmp_AN = Vec::new();
  tmp_AN.push(AN[0]);
  for i in 1..AN.len() {
    tmp_AN.push(AN[i] - AN[i-1]);
  }
  tmp_AN.push(L-AN[AN.len()-1]);
  
  // binary search
  let mut min = 1;
  let mut max = L;
  let mut half = 0;
  while min != max {
  	half = (max + min) / 2;
    half += 1;
    //println!("min:{}, max:{}, half:{}", min, max, half);
  	if solve(K, half, &tmp_AN) {
      min = half;
    } else {
      max = half - 1;
    }
  }

  println!("{}", min);
}

fn solve(k: u32, half: u64, an: &Vec<u64>) -> bool {
  let mut count = 0;
  let mut index = 0;
  let mut tmp = 0;
  while index != an.len() {
    tmp += an[index];
    index += 1;
    if tmp >= half {
      tmp = 0;
      count += 1;
    }
  }
  
  if count > k {
    true
  } else {
    false
  }
}
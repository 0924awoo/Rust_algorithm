fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut n: usize = s.trim().parse().unwrap();
    
    s.clear();
    let mut an: Vec<i64> = Vec::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut splt = s.trim().split(' ');
    for i in splt {
      let mut tmp: i64 = i.parse().unwrap();
      an.push(tmp);
    }
    an.sort();
    let an_min = an[0];
    let an_max = an[n-1];
    
    s.clear();
    std::io::stdin().read_line(&mut s).ok();
    let mut q: usize = s.trim().parse().unwrap();
    
    for i in 0..q {
       s.clear();
        std::io::stdin().read_line(&mut s).ok();
        let mut bn: i64 = s.trim().parse().unwrap();
      
      if bn <= an_min {
        println!("{}", an_min - bn);
        continue;
      } else if an_max <= bn {
        println!("{}", bn - an_max);
        continue;
      }
         
      let mut ans = binary_search(0, n-1, bn, &an);
      if ans == n-1 {
        println!("{}", min_search((an[n-1] - bn).abs(), (an[n-2] - bn).abs() ) );
      } else {
        println!("{}", min_search((an[ans] - bn).abs(), (an[ans+1] - bn).abs() ) );    
      }
        
    }
    
  }
  
  // binary_search
  // return half index
  fn binary_search(min: usize, max: usize, target: i64, an: &Vec<i64>) -> usize {
    if min == max {
        return min;
    }
    let half = (min + max) / 2 + 1;

    if target < an[half] {
        return binary_search(min, half - 1, target, an);
    } else {
        return binary_search(half, max, target, an);
    }

  }


  fn min_search(x: i64, y: i64) -> i64 {
    if x < y {
      return x;
    }
    return y;
  }
fn main() {
  let modu = 1000000000 + 7;
  let mut s = String::new();
   	std::io::stdin().read_line(&mut s).ok();
    let mut spl = s.trim().split(' ');
    let mut n: usize = spl.next().unwrap().parse().unwrap();
    let mut b: usize = spl.next().unwrap().parse().unwrap();
  let mut k: usize = spl.next().unwrap().parse().unwrap();
  
  let mut c: Vec<usize> = Vec::new();
  s.clear();
  std::io::stdin().read_line(&mut s).ok();
  let mut splt = s.trim().split(' ');
  for an in splt {
    let tmp: usize = an.parse().unwrap();
    c.push(tmp);
  }
  
  // init d_dp[][]
  let mut d_dp: Vec<Vec<usize>> = Vec::new();
  d_dp.push(Vec::new());
  for i in 0..b {
    d_dp[0].push(0);
  }
  
  // digit n
  for j in 0..k {
    d_dp[0][c[j] % b] += 1;
  }
  
  // digit n-1 ~ 1
  for i in 1..n {
    d_dp.push(Vec::new());
    for j in 0..b {
      d_dp[i].push(0);
    }
    
  	for j in 0..b {
      for m in 0..k {
    	let nex = (10*j + c[m]) % b;
        //println!("m:{}, d_dp.len:{}, nex:{}", m, d_dp.len(), nex);
        d_dp[i][nex] += d_dp[i-1][j];
        d_dp[i][nex] %= modu;
    
      }
    }
  }
  
  println!("{}", d_dp[n-1][0]);
  //for i in 0..b {
  //  println!("{}", d_dp[n-1][i]);
  //}
  
}
 
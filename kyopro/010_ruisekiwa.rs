fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).ok();
    let mut n: usize = s.trim().parse().unwrap();
    
    let mut an: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
    	s.clear();
    	std::io::stdin().read_line(&mut s).ok();
    	let mut splt = s.trim().split(' ');
      let c: usize = splt.next().unwrap().parse().unwrap();
      let p: usize = splt.next().unwrap().parse().unwrap();
      an.push((c, p));
    }
 
  //ruisekiwa
  let mut rui1: Vec<usize> = Vec::new();
  let mut rui2: Vec<usize> = Vec::new();
      rui2.push(0);
      rui1.push(0);
  
  for i in 0..n {
    if an[i].0 == 1 {
      rui1.push(rui1[i] + an[i].1);
      rui2.push(rui2[i]);
    } else {
      rui1.push(rui1[i]);
      rui2.push(rui2[i] + an[i].1);      
    }
  }
  
    s.clear();
    std::io::stdin().read_line(&mut s).ok();
  	let mut q: usize = s.trim().parse().unwrap();
  
  for i in 0..q {
    s.clear();
    std::io::stdin().read_line(&mut s).ok();
    let mut splt = s.trim().split(' ');
    let lq: usize = splt.next().unwrap().parse().unwrap();
    let rq: usize = splt.next().unwrap().parse().unwrap();

    println!("{} {}", rui1[rq] - rui1[lq-1], rui2[rq] - rui2[lq-1]);
  }
}
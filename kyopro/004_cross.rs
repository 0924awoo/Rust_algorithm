fn main() {
    let mut s = String::new();
         std::io::stdin().read_line(&mut s).ok();
      let mut spl = s.trim().split(' ');
      let mut h: usize = spl.next().unwrap().parse().unwrap();
      let mut w: usize = spl.next().unwrap().parse().unwrap();
    
    let mut row_sum: Vec<usize> = Vec::new();
    let mut column_sum: Vec<usize> = Vec::new();
    for i in 0..w {
      column_sum.push(0);
    }
    let mut cross = Vec::new();
    
    for i in 0..h {
      cross.push(Vec::new());
      let mut sum = 0;
      s.clear();
      std::io::stdin().read_line(&mut s).ok();
      let mut splt = s.trim().split(' ');
      for j in 0..w {
        let mut tmp: usize = splt.next().unwrap().parse().unwrap();
        sum += tmp;
        column_sum[j] += tmp;
        cross[i].push(tmp);
      }
      row_sum.push(sum);
    }
    
    for i in 0..h {
      for j in 0..w {
        print!("{}", row_sum[i] + column_sum[j] - cross[i][j]);
        if j != w-1 {
          print!(" ");
        } else {
          print!("\n");
        }
      }
    }
  }
   
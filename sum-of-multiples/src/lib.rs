pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    let mut a = vec![0u32];
    for i in factors {
    let mut curr_fact = *i;
    while curr_fact < limit {
        a.push(curr_fact);
       curr_fact += *i;
    }
   }
   a.sort();
   a.dedup();
   return a.iter().sum();
   
}


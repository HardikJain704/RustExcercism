pub fn is_armstrong_number(num: u64) -> bool {
    let mut number = num;
  
   let mut number_of_digit = vec![];
    
    while number > 0 {
        number_of_digit.push(number%10);
        number /= 10;
        
    }
    let mut sum:u64 = 0;
    
    let length = number_of_digit.len()as u64;
      if length == 1 {
        return true;
    }
     number_of_digit.iter().for_each(|digit|{
         let p = digit.pow(length.try_into().unwrap());
         sum += p;
         
     });
    
    if sum != num {
        return false;
    }
     true

}



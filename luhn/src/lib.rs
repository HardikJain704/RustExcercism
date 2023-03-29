/// Check a Luhn checksum.
/// 
/// 
pub fn limit(c: u32) -> u32 {
    match  c > 9 {
        true => c - 9,
        false => c,
    }
}

pub fn is_valid(code: &str) -> bool {
    let mut mul:u32  = 1;
    let mut sum:u32  = 0;
    let mut size:u32 = 0;
    for i in code.chars().into_iter().rev() {
       if i.is_ascii_whitespace(){
        continue;
       }
       sum += match i.to_digit(10){
        Some(x) => limit(x * mul),
        None => return false,
       };
       mul ^= 3;
       size += 1;
   }
   sum % 10 == 0 && size > 1
}



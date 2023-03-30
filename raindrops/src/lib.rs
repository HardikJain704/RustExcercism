pub fn raindrops(n: u32) -> String {
    let mut newstring = String::new();
    if n % 3 == 0 {
        newstring += "Pling"
    };
    if n % 5 == 0 {
        newstring += "Plang"
    };
    if n % 7 == 0 {
        newstring += "Plong"
    };
    if newstring == "" {
        return n.to_string();
    } else {
        return newstring;
    }
}

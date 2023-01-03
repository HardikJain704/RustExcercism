use std::collections::HashSet;

// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
//  let mut lower = word.to_lowercase();
//  let mut words_sorted = sorted_way(&lower);

//    possible_anagrams.iter().filter(|possible|{
//     let possible_word = possible.to_lowercase();
//     possible_word.len() == lower.len() && possible_word != lower && sorted_way(&possible_word) == words_sorted
//       })
//    .copied()
//    .collect()
// }


// fn sorted_way(word: &str) -> Vec<char> {
//     let mut words_sorted: Vec<char> = word.chars().collect();
//    words_sorted.sort_unstable();
//    words_sorted
// }


  

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let word_lower = word.to_lowercase();
    let word_sorted = get_sorted(&word_lower);
    possible_anagrams
        .iter()
        .filter(|candidate| {
            let candidate_lower = candidate.to_lowercase();
            candidate_lower.len() == word_lower.len()
                && candidate_lower != word_lower
                && get_sorted(&candidate_lower) == word_sorted
        })
        .copied()
        .collect()
}
fn get_sorted(word: &str) -> Vec<char> {
    let mut word_sorted: Vec<char> = word.chars().collect();
    word_sorted.sort_unstable();
    word_sorted
}

use std::io;

/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut letters = [false; 26]; // 26 letters in the alphabet, initally set to false
    for c in sentence.chars() {
        if let Some(i) = (c.to_ascii_lowercase() as u8).checked_sub(b'a') {
     
            // to_ascii_lowercase() converts the character to lowercase;
            // checked_sub() returns the difference between the two characters;

            if i < 26 {
                letters[i as usize] = true;
            }
        }
    }
    letters.iter().all(|&x| x)
    // letters.iter() returns an iterator over the array;
}

fn main(){
    let mut sentence = String::new();
    io::stdin().read_line(&mut sentence)
        .expect("Failed to read line");
    is_pangram(&sentence);
}
use std::io;

fn main() {
    let mut word = String::new();

    println!("Enter word");
    io::stdin().read_line(&mut word).expect("Failed to read line");

    let word = word.trim(); 
    if is_palindrome(word.to_string()) {
        println!("{} is a palindrome", word);
    } else {
        println!("{} is not a palindrome", word);
    }
}

fn is_palindrome(word: String) -> bool {
    let word = word
        .chars()
        .filter(|c| c.is_alphanumeric())
        .map(|c| c.to_lowercase().next().unwrap_or_default())
        .collect::<Vec<_>>();

    if word.is_empty() {
        return true;
    }
    let (mut left, mut right) = (0, word.len() - 1);
    while left < right {
        if word[left] != word[right] {
            return false;
        }
        left += 1;
        right -= 1;
    }
    true
}

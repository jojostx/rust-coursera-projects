//! Module that contains functions to check if a word is a
//! palindrome, count the occurrence of a specific character in a string, and reverse a string.

/// function that checks if a word is a palindrome
///
/// ```
/// use cli_utils::string_utils::is_palindrome;
///
/// let word = "omolomo";
///
/// println!("is the word {} a palindrome? answer: {}", word, is_palindrome(word));
/// assert_eq!(true, is_palindrome(word));
/// ```
pub fn is_palindrome(word: &str) -> bool {
    let rev_word: String = String::from(word).chars().rev().collect();

    rev_word == word
}

/// function that checks if a word is a palindrome
///
/// ```
/// use cli_utils::string_utils::count_char_in_string;
///
/// let word = "omolomo";
///
/// println!("there are {} letters in the word {}", count_char_in_string('o', word), word);
/// assert_eq!(4, count_char_in_string('o', word));
/// ```
pub fn count_char_in_string(c: char, word: &str) -> u32 {
    let mut char_count = 0;

    for i in String::from(word).chars() {
        if i == c {
            char_count += 1;
        }
    }

    char_count
}

/// function that checks if a word is a palindrome
///
/// ```
/// use cli_utils::string_utils::reverse_string;
///
/// let word = "omolomo";
///
/// println!("{} in reverse is {}", word, reverse_string(word));
/// assert_eq!("omolomo", reverse_string(word));
/// ```
pub fn reverse_string(word: &str) -> String {
    String::from(word).chars().rev().collect()
}

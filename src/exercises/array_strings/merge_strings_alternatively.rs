// You are given two strings word1 and word2. Merge the strings by adding letters in alternating order, starting with word1. If a string is longer than the other, append the additional letters onto the end of the merged string.

// Return the merged string.

// Example 1:

// Input: word1 = "abc", word2 = "pqr"
// Output: "apbqcr"
// Explanation: The merged string will be merged as so:
// word1:  a   b   c
// word2:    p   q   r
// merged: a p b q c r
// Example 2:

// Input: word1 = "ab", word2 = "pqrs"
// Output: "apbqrs"
// Explanation: Notice that as word2 is longer, "rs" is appended to the end.
// word1:  a   b
// word2:    p   q   r   s
// merged: a p b q   r   s
// Example 3:

// Input: word1 = "abcd", word2 = "pq"
// Output: "apbqcd"
// Explanation: Notice that as word1 is longer, "cd" is appended to the end.
// word1:  a   b   c   d
// word2:    p   q
// merged: a p b q c   d

// Constraints:

// 1 <= word1.length, word2.length <= 100
// word1 and word2 consist of lowercase English letters.

pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut result = String::new();
    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();

    loop {
        match (chars1.next(), chars2.next()) {
            (Some(c1), Some(c2)) => {
                result.push(c1);
                result.push(c2);
            }
            (Some(c1), None) => result.push(c1),
            (None, Some(c2)) => result.push(c2),
            (None, None) => break,
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_equal_length() {
        let word1 = String::from("abc");
        let word2 = String::from("pqr");
        assert_eq!(merge_alternately(word1, word2), "apbqcr");
    }

    #[test]
    fn test_word1_longer() {
        let word1 = String::from("abcd");
        let word2 = String::from("pq");
        assert_eq!(merge_alternately(word1, word2), "apbqcd");
    }

    #[test]
    fn test_word2_longer() {
        let word1 = String::from("ab");
        let word2 = String::from("pqrs");
        assert_eq!(merge_alternately(word1, word2), "apbqrs");
    }

    #[test]
    fn test_empty_word1() {
        let word1 = String::from("");
        let word2 = String::from("pqr");
        assert_eq!(merge_alternately(word1, word2), "pqr");
    }

    #[test]
    fn test_empty_word2() {
        let word1 = String::from("abc");
        let word2 = String::from("");
        assert_eq!(merge_alternately(word1, word2), "abc");
    }

    #[test]
    fn test_both_empty() {
        let word1 = String::from("");
        let word2 = String::from("");
        assert_eq!(merge_alternately(word1, word2), "");
    }
}

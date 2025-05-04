// Problem: Construction Bar Length Finder
// 
// You are working on a construction project and need to find two metal bars that together
// have exactly the length needed for a specific part of the structure. You have a collection of
// bars with different lengths.
//
// Given an array of bar lengths (in centimeters) and a target length, find the indices of two bars
// that, when combined, have exactly the target length. Each bar can only be used once, and you
// can assume there is exactly one valid solution.
//
// Example:
// Input: bar_lengths = [120, 235, 80, 185, 150], target_length = 300
// Output: [0, 4]
// Explanation: The bars at indices 0 and 4 have lengths 120 and 150, which sum to 270.


pub fn find_bar_pair(bar_lengths: Vec<i32>, target_length: i32) -> Vec<i32> {
    // Your code here
    vec![] // Replace this with your solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let bar_lengths = vec![120, 235, 80, 185, 150];
        let target_length = 270;
        let result = find_bar_pair(bar_lengths, target_length);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&0) && result.contains(&4));
    }

    #[test]
    fn test_case_2() {
        let bar_lengths = vec![50, 75, 100, 125, 150, 175];
        let target_length = 200;
        let result = find_bar_pair(bar_lengths, target_length);
        assert_eq!(result.len(), 2);
        assert!((result.contains(&0) && result.contains(&5)) || 
                (result.contains(&2) && result.contains(&3)));
    }

    #[test]
    fn test_case_3() {
        let bar_lengths = vec![300, 250, 200, 150, 100];
        let target_length = 350;
        let result = find_bar_pair(bar_lengths, target_length);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&1) && result.contains(&3));
    }
}
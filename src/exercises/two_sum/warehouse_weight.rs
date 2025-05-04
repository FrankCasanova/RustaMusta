// Problem: Warehouse Box Weight Matcher
// 
// You are managing a warehouse with boxes of different weights. A customer needs exactly two boxes
// that together have a specific target weight for their shipping container. Write a function that
// finds the indices of two boxes that add up to the target weight.
//
// Given an array of box weights and a target weight, return the indices of the two boxes
// that add up exactly to the target weight. You may assume there is exactly one solution,
// and you cannot use the same box twice.
//
// Example:
// Input: box_weights = [150, 210, 80, 375, 160], target_weight = 440
// Output: [1, 4]
// Explanation: The boxes at indices 1 and 4 have weights 210 and 160, which sum to 370.


use std::collections::HashMap;

pub fn find_box_pair(box_weights: Vec<i32>, target_weight: i32) -> Vec<i32> {

    // This should never happen as we're guaranteed a solution
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let box_weights = vec![150, 210, 80, 375, 160];
        let target_weight = 370;
        let result = find_box_pair(box_weights, target_weight);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&1) && result.contains(&4));
    }

    #[test]
    fn test_case_2() {
        let box_weights = vec![45, 30, 25, 60, 75, 15];
        let target_weight = 105;
        let result = find_box_pair(box_weights, target_weight);
        assert_eq!(result.len(), 2);
        assert!((result.contains(&0) && result.contains(&4)) || 
                (result.contains(&1) && result.contains(&3)));
    }

    #[test]
    fn test_case_3() {
        let box_weights = vec![500, 250, 750, 1000];
        let target_weight = 1000;
        let result = find_box_pair(box_weights, target_weight);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&0) && result.contains(&2));
    }
}
// Problem: Two Sum 
// 
// Given an array of integers nums and an integer target, return indices of the two numbers 
// such that they add up to target. You may assume that each input would have exactly one solution, 
// and you may not use the same element twice.
//
// You can return the answer in any order.
//
// Example:
// Input: nums = [2, 7, 11, 15], target = 9
// Output: [0, 1]
// Explanation: Because nums[0] + nums[1] == 9, we return [0, 1]

// Solution is implemented now!

use std::collections::HashMap;

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    
    for (i, &num) in nums.iter().enumerate() {
        let complement = target - num;
        
        if map.contains_key(&complement) {
            return vec![map[&complement], i as i32];
        }
        
        map.insert(num, i as i32);
    }
    
    // This should never happen as we're guaranteed a solution
    vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }

    #[test]
    fn test_case_2() {
        let nums = vec![3, 2, 4];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![1, 2]);
    }

    #[test]
    fn test_case_3() {
        let nums = vec![3, 3];
        let target = 6;
        assert_eq!(two_sum(nums, target), vec![0, 1]);
    }
}
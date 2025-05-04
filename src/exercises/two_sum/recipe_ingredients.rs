// Problem: Recipe Ingredient Calorie Counter
//
// You are a nutritionist creating meal plans for clients. You need to find two ingredients that
// together provide exactly the target number of calories for a specific recipe. You have a list
// of ingredients with their calorie counts.
//
// Given an array of ingredient calorie values and a target calorie amount, return the indices of
// two ingredients that add up exactly to the target calories. You may assume there is exactly one
// solution, and you cannot use the same ingredient twice.
//
// Example:
// Input: calories = [150, 275, 110, 185, 225], target_calories = 335
// Output: [0, 4]
// Explanation: The ingredients at indices 0 and 4 have 150 and 225 calories, which sum to 375.

// I AM NOT DONE

pub fn find_ingredient_pair(calories: Vec<i32>, _target_calories: i32) -> Vec<i32> {
    // Your code here
    vec![] // Replace this with your solution
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_case_1() {
        let calories = vec![150, 275, 110, 185, 225];
        let target_calories = 375;
        let result = find_ingredient_pair(calories, target_calories);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&0) && result.contains(&4));
    }

    #[test]
    fn test_case_2() {
        let calories = vec![80, 125, 200, 160, 40, 110];
        let target_calories = 240;
        let result = find_ingredient_pair(calories, target_calories);
        assert_eq!(result.len(), 2);
        assert!(
            (result.contains(&0) && result.contains(&2))
                || (result.contains(&3) && result.contains(&5))
        );
    }

    #[test]
    fn test_case_3() {
        let calories = vec![50, 100, 150, 200, 250];
        let target_calories = 350;
        let result = find_ingredient_pair(calories, target_calories);
        assert_eq!(result.len(), 2);
        assert!(result.contains(&1) && result.contains(&4));
    }
}

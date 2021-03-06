// Given an array of integers nums and an integer target, return indices of the two numbers such that they add up to target.

// You may assume that each input would have exactly one solution, and you may not use the same element twice.

// You can return the answer in any order.

// Example 1:

// Input: nums = [2,7,11,15], target = 9
// Output: [0,1]
// Output: Because nums[0] + nums[1] == 9, we return [0, 1].

// Example 2:

// Input: nums = [3,2,4], target = 6
// Output: [1,2]

// Example 3:

// Input: nums = [3,3], target = 6
// Output: [0,1]

// Constraints:

//     2 <= nums.length <= 104
//     -109 <= nums[i] <= 109
//     -109 <= target <= 109
//     Only one valid answer exists.
use std::collections::HashMap;

struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        // leetcode being leetcode
        // if len < 2 || len > 204 {
        //     return Vec::new();
        // }

        let mut lookup = HashMap::new();
        for (i, n) in nums.iter().enumerate() {
            let diff = target - n;
            if lookup.contains_key(&diff) {
                return vec![*lookup.get(&diff).unwrap() as i32, i as i32];
            }
            lookup.insert(n, i);
        }

        return Vec::new();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution() {
        let nums = vec![2, 7, 11, 15];
        let target = 9;
        let correct_result = vec![0, 1];
        let result = Solution::two_sum(nums, target);
        assert_eq!(result, correct_result);
    }
}

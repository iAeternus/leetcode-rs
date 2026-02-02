//! <https://leetcode.cn/problems/longest-consecutive-sequence>
use std::collections::HashSet;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let nums: HashSet<i32> = nums.iter().cloned().collect();
        let mut longest = 0;

        for &num in &nums {
            // num - 1不存在，num为当前序列的开始
            if !nums.contains(&(num - 1)) {
                let mut curr_num = num;
                let mut curr_len = 1;
                while nums.contains(&(curr_num + 1)) {
                    curr_num += 1;
                    curr_len += 1;
                }
                longest = longest.max(curr_len);
            }
        }

        longest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(Solution::longest_consecutive(nums), 4);
    }

    #[test]
    fn test2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(Solution::longest_consecutive(nums), 9);
    }

    #[test]
    fn test3() {
        let nums = vec![1, 0, 1, 2];
        assert_eq!(Solution::longest_consecutive(nums), 3);
    }
}

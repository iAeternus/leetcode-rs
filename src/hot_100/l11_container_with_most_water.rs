//! <https://leetcode.cn/problems/container-with-most-water>
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut max_s = 0;

        while left < right {
            max_s = max_s.max(height[left].min(height[right]) * (right - left) as i32);
            if height[left] < height[right] {
                left += 1;
            } else {
                right -= 1;
            }
        }

        max_s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
        assert_eq!(Solution::max_area(height), 49);
    }

    #[test]
    fn test2() {
        let height = vec![1, 1];
        assert_eq!(Solution::max_area(height), 1);
    }
}

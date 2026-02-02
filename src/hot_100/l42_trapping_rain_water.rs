//! <https://leetcode.cn/problems/trapping-rain-water>
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    /// 法一：dp，对每个位置预先计算其左右最大高度
    /// - 时间复杂度：O(n)
    /// - 空间复杂度：O(n)
    pub fn trap(height: Vec<i32>) -> i32 {
        let n = height.len();
        if n < 3 {
            return 0;
        }

        let mut left_max = vec![0; n];
        let mut right_max = vec![0; n];

        left_max[0] = height[0];
        for i in 1..n {
            left_max[i] = height[i].max(left_max[i - 1]);
        }

        right_max[n - 1] = height[n - 1];
        for i in (0..n - 1).rev() {
            right_max[i] = height[i].max(right_max[i + 1]);
        }

        let mut res = 0;
        for i in 0..n {
            res += left_max[i].min(right_max[i]) - height[i];
        }

        res
    }

    /// 法二：单调栈，维护高度递减的柱子索引，遇到更高柱子时，计算前一个低洼处与左右边界形成的积水
    /// - 时间复杂度：O(n)
    /// - 空间复杂度：O(n)
    pub fn trap2(height: Vec<i32>) -> i32 {
        let mut stack = Vec::new(); // 存储索引
        let mut res = 0;

        for (i, &cur_h) in height.iter().enumerate() {
            while !stack.is_empty() && height[stack[stack.len() - 1]] <= cur_h {
                let bottom_h = height[stack.pop().unwrap()];
                if stack.is_empty() {
                    break;
                }
                let left = stack[stack.len() - 1];
                let h = cur_h.min(height[left]) - bottom_h;
                let d = (i - left - 1) as i32;
                res += d * h;
            }
            stack.push(i);
        }

        res
    }

    /// 法三：双指针，左右指针向中间移动，动态记录左右最大高度，每次移动较小高度的指针并计算当前位置积水量
    /// - 时间复杂度：O(n)
    /// - 空间复杂度：O(1)
    pub fn trap3(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut l_max = 0;
        let mut r_max = 0;
        let mut res = 0;

        while left < right {
            if height[left] < height[right] {
                if height[left] > l_max {
                    l_max = height[left];
                } else {
                    res += l_max - height[left];
                }
                left += 1;
            } else {
                if height[right] > r_max {
                    r_max = height[right];
                } else {
                    res += r_max - height[right];
                }
                right -= 1;
            }
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
        assert_eq!(Solution::trap(height.clone()), 6);
        assert_eq!(Solution::trap2(height.clone()), 6);
        assert_eq!(Solution::trap3(height.clone()), 6);
    }

    #[test]
    fn test2() {
        let height = vec![4, 2, 0, 3, 2, 5];
        assert_eq!(Solution::trap(height.clone()), 9);
        assert_eq!(Solution::trap2(height.clone()), 9);
        assert_eq!(Solution::trap3(height.clone()), 9);
    }
}

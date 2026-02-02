//! <https://leetcode.cn/problems/3sum>
pub struct Solution;

// 本题也可以使用HashSet来帮助去重，这样就没有这么多去重trick
#[allow(dead_code)]
impl Solution {
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut nums = nums;
        nums.sort_unstable();
        let n = nums.len();
        let mut res = Vec::new();

        for i in 0..n - 2 {
            let x = nums[i];
            if i > 0 && x == nums[i - 1] {
                continue; // 相同的数会产生相同的三元组
            }
            if x + nums[i + 1] + nums[i + 2] > 0 {
                break; // x过大，停止
            }
            if x + nums[n - 1] + nums[n - 2] < 0 {
                continue; // x过小，继续寻找
            }

            let target = -nums[i];
            let mut left = i + 1;
            let mut right = n - 1;

            while left < right {
                let sum = nums[left] + nums[right];
                if sum < target {
                    left += 1;
                } else if sum > target {
                    right -= 1;
                } else {
                    if left == i + 1 || nums[left] != nums[left - 1] {
                        res.push(vec![nums[i], nums[left], nums[right]]); // 去重相同元组
                    }
                    left += 1;
                    right -= 1;
                }
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
        let nums = vec![-1, 0, 1, 2, -1, -4];
        assert_eq!(
            Solution::three_sum(nums),
            vec![vec![-1, -1, 2], vec![-1, 0, 1]]
        );
    }

    #[test]
    fn test2() {
        let nums = vec![0, 1, 1];
        assert_eq!(Solution::three_sum(nums), Vec::<Vec<i32>>::new());
    }

    #[test]
    fn test3() {
        let nums = vec![0, 0, 0];
        assert_eq!(Solution::three_sum(nums), vec![vec![0, 0, 0]]);
    }

    #[test]
    fn test4() {
        let nums = vec![-2, 0, 0, 2, 0, 2, 2];
        assert_eq!(
            Solution::three_sum(nums),
            vec![vec![-2, 0, 2], vec![0, 0, 0]]
        );
    }
}

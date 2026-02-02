//! <https://leetcode.cn/problems/group-anagrams>
use std::collections::HashMap;

pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for str in strs {
            let mut chars: Vec<char> = str.chars().collect();
            chars.sort_unstable();
            let key = chars.into_iter().collect();
            map.entry(key).or_default().push(str); // or_default是惰性求值，只有key不存在时才调用Vec::default
        }
        map.into_values().collect()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    #[test]
    fn test() {
        let strs = vec!["eat", "tea", "tan", "ate", "nat", "bat"]
            .iter()
            .map(|&str| str.into())
            .collect();

        let res = Solution::group_anagrams(strs); // 可能以任意顺序返回

        let res: BTreeSet<BTreeSet<String>> = res
            .into_iter()
            .map(|group| group.into_iter().collect())
            .collect();

        let expect: BTreeSet<BTreeSet<String>> =
            vec![vec!["bat"], vec!["nat", "tan"], vec!["ate", "eat", "tea"]]
                .into_iter()
                .map(|group| group.into_iter().map(|str| str.into()).collect())
                .collect();

        assert_eq!(res, expect);
    }
}

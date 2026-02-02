pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack = Vec::new();
        for c in s.chars() {
            match c {
                '(' | '[' | '{' => stack.push(c),
                ')' | ']' | '}' => {
                    if let Some(top) = stack.pop() {
                        if c == ')' && top != '('
                            || c == ']' && top != '['
                            || c == '}' && top != '{'
                        {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
                _ => unreachable!(),
            }
        }
        stack.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert!(Solution::is_valid("()".into()));
        assert!(Solution::is_valid("()[]{}".into()));
        assert!(!Solution::is_valid("(]".into()));
        assert!(Solution::is_valid("([])".into()));
        assert!(!Solution::is_valid("([)]".into()));
    }
}

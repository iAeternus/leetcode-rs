use leetcode_rs::{LineReader, input};

struct Solution;

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

fn main() {
    let mut r = LineReader::new();
    let s = input!(r, String);
    println!("{}", Solution::is_valid(s));
}

use std::io::{self, BufRead, BufReader};
use std::str::FromStr;

pub struct LineReader {
    buffer: Vec<String>,
    reader: BufReader<io::Stdin>,
}

#[allow(dead_code)]
impl LineReader {
    pub fn new() -> Self {
        let reader = BufReader::new(io::stdin());
        LineReader {
            buffer: Vec::new(),
            reader,
        }
    }

    /// 读取下一个标记并解析为指定类型
    pub fn next<T: FromStr>(&mut self) -> Option<T> {
        while self.buffer.is_empty() {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => return None, // EOF
                Ok(_) => {
                    self.buffer = line.split_whitespace().map(|s| s.to_string()).collect();
                }
                Err(_) => return None,
            }
        }

        let token = self.buffer.remove(0);
        token.parse::<T>().ok()
    }

    /// 强制读取并解析，如果失败会panic
    pub fn next_parse<T: FromStr>(&mut self) -> T {
        self.next().expect("Failed to parse input")
    }

    /// 读取整行并解析为向量
    pub fn next_vec<T: FromStr>(&mut self) -> Option<Vec<T>> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => {
                let vec: Result<Vec<T>, _> = line.split_whitespace().map(|s| s.parse()).collect();
                vec.ok()
            }
            Err(_) => None,
        }
    }

    /// 读取整行作为字符串（保留空白字符）
    pub fn next_line(&mut self) -> Option<String> {
        let mut line = String::new();
        match self.reader.read_line(&mut line) {
            Ok(0) => None,
            Ok(_) => {
                if line.ends_with('\n') {
                    line.pop();
                    if line.ends_with('\r') {
                        line.pop();
                    }
                }
                Some(line)
            }
            Err(_) => None,
        }
    }

    /// 读取指定数量的元素
    pub fn next_n<T: FromStr>(&mut self, n: usize) -> Option<Vec<T>> {
        let mut result = Vec::with_capacity(n);
        for _ in 0..n {
            match self.next() {
                Some(value) => result.push(value),
                None => return None,
            }
        }
        Some(result)
    }

    /// 检查是否还有更多输入
    pub fn has_next(&mut self) -> bool {
        !self.buffer.is_empty() || {
            let mut line = String::new();
            match self.reader.read_line(&mut line) {
                Ok(0) => false,
                Ok(_) => {
                    self.buffer = line.split_whitespace().map(|s| s.to_string()).collect();
                    !self.buffer.is_empty()
                }
                Err(_) => false,
            }
        }
    }

    /// 读取到特定分隔符（如逗号分隔）
    pub fn next_delimited<T: FromStr>(&mut self, delimiter: char) -> Option<Vec<T>> {
        let line = self.next_line()?;
        let vec: Result<Vec<T>, _> = line.split(delimiter).map(|s| s.trim().parse()).collect();
        vec.ok()
    }
}

#[macro_export]
macro_rules! input {
    ($reader:expr, $($t:ty),+) => {
        ($($reader.next_parse::<$t>()),+)
    };

    ($reader:expr, $($t:ty),+) => {
        ($($reader.next_parse::<$t>()),+)
    };
}

#[macro_export]
macro_rules! input_vec {
    // 读取向量（整行）
    ($reader:expr, $t:ty) => {
        $reader.next_vec::<$t>().unwrap()
    };

    // 读取特定长度的向量
    ($reader:expr, $t:ty, $n:expr) => {
        $reader.next_n::<$t>($n).unwrap()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "Including blocking input"]
    fn test() {
        let mut r = LineReader::new();

        // // 读取整数个数
        // let n = input!(r, usize);

        // // 读取一行整数
        // let nums = input_vec!(r, i32, n);

        // let sum: i32 = nums.iter().sum();
        // println!("{}", sum);

        // // 读取字符串
        // let s = input!(r, String);
        // println!("{}", s);

        // 读取矩阵
        let (n, m) = input!(r, usize, usize);
        let mut mat = Vec::new();
        (0..n).for_each(|_| mat.push(input_vec!(r, i32, m)));

        let sum: i32 = mat.iter().flatten().sum();
        println!("{}", sum);
    }
}

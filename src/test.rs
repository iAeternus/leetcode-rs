use leetcode_rs::{LineReader, input, input_vec};

fn main() {
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

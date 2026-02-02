use std::{io, str::FromStr};

pub struct LineReader {
    input: Vec<String>,
    index: usize,
}

impl LineReader {
    pub fn new() -> Self {
        let mut reader = LineReader {
            input: Vec::new(),
            index: 0,
        };
        reader.go_next();
        reader
    }

    pub fn next<T: FromStr>(&mut self) -> T {
        let res = self.input[self.index].trim().parse::<T>();
        self.index += 1;
        if self.index >= self.input.len() {
            self.go_next();
        }
        res.ok().unwrap()
    }

    pub fn next_vec<T: FromStr>(&mut self) -> Vec<T> {
        let res = self
            .input
            .iter()
            .map(|x| {
                x.parse::<T>()
                    .map_err(|_| io::Error::new(io::ErrorKind::InvalidInput, "Failed"))
            })
            .collect::<Result<Vec<T>, _>>()
            .unwrap();
        self.go_next();
        res
    }

    fn go_next(&mut self) {
        self.clear();
        self.read();
    }

    fn clear(&mut self) {
        self.input.clear();
        self.index = 0;
    }

    fn read(&mut self) {
        let stdin = io::stdin();
        let mut buf = String::new();
        stdin.read_line(&mut buf).unwrap();
        self.input = buf.split_whitespace().map(|s| s.to_owned()).collect();
        self.index = 0;
    }
}

//! ファイル群を段階的にシミュレートする
#![allow(dead_code)]

use std::fmt;
use std::fmt::{Display};

/// file は、アクセス可能なファイルを意味する
#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed,
}

#[derive(Debug)]
pub struct File {
    pub name: String,
    data: Vec<u8>,
    state: FileState,
}

impl Display for FileState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

impl Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<{}({})>", self.name, self.state)
    }
}

impl File {
    /// 新規ファイルは空とみなすが、ファイル名は必須
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed,
        }
    }

    fn read(
        self: &File,
        save_to: &mut Vec<u8>
    ) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for readinng"));
        }
        let mut tmp = self.data.clone();
        let read_length = tmp.len();
        save_to.reserve(read_length);
        save_to.append(&mut tmp);
        Ok(read_length)
    }
}

fn open(mut f: File) -> Result<File, String> {
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let mut f = File::new("5.txt");

    let mut buffer: Vec<u8> = vec![];

    if f.read(&mut buffer).is_err() {
        println!("Error checking is working");
    }

    f = open(f).unwrap();
    let f3_length = f.read(&mut buffer).unwrap();
    f = close(f).unwrap();

    let text = String::from_utf8_lossy(&buffer);

    println!("{:?}", f);
    println!("{}", f);
    println!("{} is {} byte long", &f.name, f3_length);
    println!("{}", text);
}

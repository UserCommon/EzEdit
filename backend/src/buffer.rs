use std::fs::{read, read_to_string, write, File};
use std::io::prelude::*;
use std::path::Path;

/// Abstraction that represents rows in document
#[derive(Debug)]
pub struct Row {
    pub row: String,
}

impl std::default::Default for Row {
    fn default() -> Self {
        Self {
            row: "".to_string(),
        }
    }
}

impl Row {
    fn new(row: String) -> Self {
        Self { row }
    }
}

/// Abstraction that represents document it self
#[derive(Debug)]
pub struct Buffer {
    pub name: String,
    buf: Vec<Row>,
    pub path: String,
}

impl std::default::Default for Buffer {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            buf: vec![],
            path: "".to_string(),
        }
    }
}

impl BufferBasicApi for Buffer {}

impl Buffer {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let os_path = Path::new(&path);
        Ok(Self {
            name: os_path
                .file_name()
                .expect("failed to get file name")
                .to_str()
                .expect("failed to convert Path to str")
                .to_string(),
            buf: Self::read_from(&path)?,
            path: path.into(),
        })
    }
}

pub trait DocumentOperations {
    fn read_from(path: &str) -> Result<Vec<Row>, std::io::Error>;
    fn write_to(&self) -> Result<(), std::io::Error>;
}

pub trait BufferBasicApi<T: std::convert::Into<String>> {
    //create
    fn create_row(&mut self, row: usize);
    fn insert_to_row(&mut self, idx: usize);
    //delete
    fn delete_row(&mut self, row: usize);
    fn delete_in_row(&mut self, range: std::ops::Range<usize>);
    // read
    fn read_row(&self, row: usize) -> Row;
    fn read_buf(&self) -> Self;
}

pub trait BufferApi<T: std::convert::Into<String>>: BufferBasicApi<T> {
    //
    //
}

impl DocumentOperations for Buffer {
    fn read_from(path: &str) -> Result<Vec<Row>, std::io::Error> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut doc_vec: Vec<Row> = vec![];
        for row in content.split('\n') {
            let row = Row::new(row.to_string());
            doc_vec.push(row);
        }
        Ok(doc_vec)
    }

    fn write_to(&self) -> Result<(), std::io::Error> {
        // FIXME! extra \n in end of file
        let mut file = File::options().write(true).open(&self.path)?;

        let mut content = String::new();

        for row in &self.buf {
            content.push_str(&row.row);
            content.push('\n');
        }

        file.write_all(content.as_bytes())?;

        Ok(())
    }
}

#[cfg(test)]
mod buffer_test {
    use std::{fs, io::Read};

    use crate::buffer::DocumentOperations;

    #[test]
    fn read_file_to_buffer() {
        use crate::buffer::{Buffer, Row};

        let content: Vec<Row> = vec![
            Row::new("Hello everyone!".to_string()),
            Row::new("How are you?".to_string()),
            Row::new("Thanks, I'm fine!".to_string()),
        ];

        let mut buf = Buffer::new("tests/read_file_to_buffer.txt").unwrap();
        assert_eq!("read_file_to_buffer.txt", buf.name);
        assert!(content.iter().zip(buf.buf).all(|(a, b)| a.row == b.row));
    }

    #[test]
    fn write_buffer_to_file() {
        use crate::buffer::{Buffer, Row};
        use std::fs::{self, File};
        File::create("tests/write_buffer_to_file.txt").unwrap();
        let mut buf = Buffer::new("tests/read_file_to_buffer.txt").unwrap();

        let mut buf_main = Buffer::new("tests/write_buffer_to_file.txt").unwrap();
        buf_main.buf = buf.buf;
        println!("{:#?}", buf_main.buf);
        buf_main.write_to().unwrap();

        let mut file_w = File::open("tests/write_buffer_to_file.txt").unwrap();
        let mut file_r = File::open("tests/read_file_to_buffer.txt").unwrap();

        let mut content_w = String::new();
        let mut content_r = String::new();

        file_w.read_to_string(&mut content_w).unwrap();
        file_r.read_to_string(&mut content_r).unwrap();

        assert_eq!(content_w.trim(), content_r.trim());
    }
}

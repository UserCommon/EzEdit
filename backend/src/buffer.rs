use std::fs::{read, read_to_string, write, File};
use std::io::prelude::*;
use std::path::Path;

/// Abstraction that represents rows in document
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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

impl BufferBasicApi for Buffer {
    fn create_row(&mut self, row: usize) {
        self.buf.insert(row, Row::default());
    }

    fn insert_to_row(&mut self, row: usize, idx: usize, substr: &str) {
        self.buf[row].row.insert_str(idx, substr);
    }

    fn delete_row(&mut self, row: usize) {
        self.buf.remove(row);
    }

    fn delete_in_row(&mut self, row: usize, range: std::ops::Range<usize>) {
        self.buf[row].row.replace_range(range, "");
    }

    fn read_row(&self, row: usize) -> Row {
        self.buf[row].clone()
    }

    fn read_buf(&self) -> Self {
        // I think that's not optimal solve :D Todo!
        (*self).clone()
    }

    fn read_as_string(&self) -> String {
        // Move to Into<String>
        let mut res = String::new();
        for s in self.buf.clone() {
            res.push_str(&format!("{}\n", s.row));
        }
        res
    }
}

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

/// BufferBasicApi is a trait that defines main methods
/// that in future will be used as foundation for
/// BufferApi that defines methods that will be
/// used in RPC Api
pub trait BufferBasicApi {
    /// Creates row, shifts other rows
    fn create_row(&mut self, row: usize);
    /// Inserts substr in row from idx position
    fn insert_to_row(&mut self, row: usize, idx: usize, substr: &str);
    /// Deletes row
    fn delete_row(&mut self, row: usize);
    /// Deletes substring in row given range
    fn delete_in_row(&mut self, row: usize, range: std::ops::Range<usize>);
    /// returns Row structure in given row
    fn read_row(&self, row: usize) -> Row;
    /// returns itself as Structure
    fn read_buf(&self) -> Self;
    /// returns itself as String representation
    fn read_as_string(&self) -> String;
}

/// BufferApi is a trait that defines main methods
/// for working with buffer
pub trait BufferApi: BufferBasicApi {
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

    use crate::buffer::*;
    use std::{fs, io::Read};

    #[test]
    fn read_file_to_buffer() {
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

    #[test]
    fn create_insert_row_test_1() {
        let mut buf = Buffer::new("tests/BufferBasicApiTest.txt").unwrap();
        let cmp_buf = Buffer::new("tests/BufferBasicApiCreateInsert.txt").unwrap();
        buf.create_row(4);
        buf.insert_to_row(4, 0, "My name is Cyril!");
        assert_eq!(buf.read_as_string(), cmp_buf.read_as_string());
    }

    #[test]
    fn delete_row_test_1() {
        let mut buf = Buffer::new("tests/BufferBasicApiTest.txt").unwrap();
        let cmp_buf = Buffer::new("tests/BufferBasicApiDeleteRowTest.txt").unwrap();

        buf.delete_row(2);
        assert_eq!(buf.read_as_string(), cmp_buf.read_as_string());
    }

    #[test]
    fn delete_in_row_test_1() {
        let mut buf = Buffer::new("tests/BufferBasicApiTest.txt").unwrap();
        let cmp_buf = Buffer::new("tests/BufferBasicApiDeleteInRowTest.txt").unwrap();

        buf.delete_in_row(2, 5..15);
        assert_eq!(buf.read_as_string(), cmp_buf.read_as_string())
    }
}

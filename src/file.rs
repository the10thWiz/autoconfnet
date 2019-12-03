use std::fs;
use std::io::{ErrorKind, Read};
/**
 * Struct for generic file reading
 *
 * Can be read using a variety of iterators
 */
pub struct File {
    name: String,
    file: fs::File,
}

impl File {
    pub fn read(name: &str) -> std::io::Result<File> {
        Ok(File {
            name: name.to_owned(),
            file: fs::File::open(name)?,
        })
    }
}

impl std::iter::Iterator for File {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        let mut ret = String::default();
        loop {
            let mut buf = [0u8];
            match self.file.read(&mut buf) {
                Ok(n) => {
                    if n <= 0 {
                        return None;
                    }
                }
                Err(e) => match e.kind() {
                    ErrorKind::UnexpectedEof => return None,
                    _ => panic!("Something happened: {:?}", e),
                },
            }
            if buf[0] as char == '\n' {
                return Some(ret);
            } else {
                ret.push(buf[0] as char);
            }
        }
    }
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "File: {}", self.name)
    }
}

use std::{path::Path, fs, io::Error};

use crate::classpath::classpath::ClasspathEntry;

pub struct DirEntry {
    abs_dir: String
}

pub fn new_direntry(abs_dir: &str) -> DirEntry {
    DirEntry {
        abs_dir: String::from(abs_dir)
    }
}

impl ClasspathEntry for DirEntry {
    fn string(&self) -> &str {
        &self.abs_dir
    }

    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, &'static str> {
        let class_filepath = Path::new(&self.abs_dir).join(class_name);
        match fs::read(class_filepath) {
            Ok(content) => Ok(content),
            Err(err) => Err(err.to_string().as_str())
        }
    }
}

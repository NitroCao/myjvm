use std::{fs::File, path::Path, io::{BufReader, Read}};

use super::classpath::ClasspathEntry;

pub struct ZipEntry {
    abs_path: String
}

pub fn new_zipentry(abs_dir: &str) -> ZipEntry {
    ZipEntry {
        abs_path: String::from(abs_dir)
    }
}

impl ClasspathEntry for ZipEntry {
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, &'static str> {
        let file = match File::open(Path::new(&self.abs_path)) {
            Ok(file) =>  file ,
            Err(err) => {
                return Err(err.to_string().as_str()); }
        };
        let reader = BufReader::new(file);
        let mut zip = match zip::ZipArchive::new(reader) {
            Ok(zip) => zip,
            Err(err) => { return Err(err.to_string().as_str());}
        };
        let mut class_file = match zip.by_name(class_name) {
            Ok(file) => file,
            Err(err) => { return Err(err.to_string().as_str());}
        };
        let mut buff: Vec<u8> = vec![0; class_file.size() as usize];
        match class_file.read_exact(buff.as_mut()) {
            Ok(()) => { Ok(buff) },
            Err(err) => { return Err(err.to_string().as_str()); }
        }
    }

    fn string(&self) -> &str {
        &self.abs_path
    }
}

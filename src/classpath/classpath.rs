pub trait ClasspathEntry {
    fn string(&self) -> &str;
    fn read_class(&self, class_name: &str) -> Result<Vec<u8>, &'static str>;
}

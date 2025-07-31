use std::fs::File;
use std::io::{ErrorKind, Write};

use std::fs::OpenOptions;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .append(true)
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    file.write(content.as_bytes()).unwrap();
}

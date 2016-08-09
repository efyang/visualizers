use std::env::temp_dir;
use std::path::PathBuf;
use std::fs::{File, remove_file};

lazy_static! {
    static ref TMPDIR: PathBuf = temp_dir().to_path_buf();
}

pub struct LockFile {
    pb: PathBuf
}

impl LockFile {
    pub fn new(name: &str) -> Option<Self> {
        let path = TMPDIR.join(name);
        if path.exists() {
            None
        } else {
            match File::create(&path) {
                Ok(_) => Some(LockFile {
                    pb: path.to_path_buf()
                }),
                Err(_) => None
            }
        }
    }
}

impl Drop for LockFile {
    fn drop(&mut self) {
        remove_file(&self.pb).unwrap();
    }
}

use std::fs;

pub struct Directory {
    pub path: String,
    pub exists: bool,
}

impl Directory {
    pub fn new(path: String) -> Self {
        let exists: bool = match fs::metadata(&path) {
            Ok(metadata) => metadata.is_dir(),
            Err(_) => false
        };

        Self {
            path,
            exists
        }
    }
}

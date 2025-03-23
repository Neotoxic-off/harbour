use std::fs;

pub struct File {
    pub path: String,
    pub exists: bool,
}

impl File {
    pub fn new(path: String) -> Self {
        Self {
            exists: fs::metadata(&path).is_ok(),
            path,
        }
    }
}

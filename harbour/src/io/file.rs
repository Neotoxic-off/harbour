use std::path::Path;

pub struct File {
    pub path: String,
    pub exists: bool
}

impl File {
    pub fn new(path: String) -> Self {
        Self {
            exists: Path::new(&path).exists(),
            path
        }
    }
}

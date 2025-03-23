use std::path::Path;

pub struct Directory {
    pub path: String,
    pub exists: bool
}

impl Directory {
    pub fn new(path: String) -> Self {
        Self {
            exists: Path::new(&path).is_dir(),
            path
        }
    }
}

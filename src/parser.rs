mod parser;

pub struct Extension(String);

impl Extension {
    pub fn new(ext: &str) -> Self {
        Extension(ext.to_string())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn slug(&self) -> String {
        self.0.replace('.', "")
    }
}

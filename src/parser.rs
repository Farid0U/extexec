// Extension handling utilities

pub struct Extension(String);

impl Extension {
    pub fn new(raw: &str) -> Self {
        let mut cleaned = raw.trim().to_lowercase();
        
        if !cleaned.starts_with('.') && !cleaned.is_empty() {
            cleaned.insert(0, '.');
        }
        
        Extension(cleaned)
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn slug(&self) -> String {
        self.0.replace('.', "")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_normalization() {
        assert_eq!(Extension::new("txt").as_str(), ".txt");
        assert_eq!(Extension::new(".TXT").as_str(), ".txt");
        assert_eq!(Extension::new("  .Png  ").as_str(), ".png");
    }
}
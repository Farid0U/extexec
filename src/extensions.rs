// Extension handling utilities
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Extension(String);

impl Extension {
    pub fn new(raw: &str) -> Self {
        let mut cleaned = raw.trim().to_lowercase();
        
        if !cleaned.starts_with('.') && !cleaned.is_empty() {
            cleaned.insert(0, '.');
        }
        
        Extension(cleaned)
    }

    pub fn slug(&self) -> String {
        self.0.trim_start_matches('.').to_string()
    }
}

impl fmt::Display for Extension {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for Extension {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Extension::new(s))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extension_normalization() {
        assert_eq!(Extension::new("txt").to_string(), ".txt");
        assert_eq!(Extension::new(".TXT").to_string(), ".txt");
        assert_eq!(Extension::new("  .Png  ").to_string(), ".png");
    }
}
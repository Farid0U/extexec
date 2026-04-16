use std::path::Path;
use std::io;

/// Validates that the provided path exists and is a file.
pub fn validate_executable(path: &Path) -> io::Result<()> {
    if !path.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("The path {:?} does not exist.", path),
        ));
    }

    if !path.is_file() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            format!("The path {:?} is a directory, not an executable file.", path),
        ));
    }

    Ok(())
}
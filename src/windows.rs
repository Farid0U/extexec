use std::path::PathBuf;
use winreg::enums::*;
use winreg::RegKey;
use std::io;
use crate::extensions::Extension;

pub fn associate_extension(ext: &Extension, exe_path: PathBuf) -> io::Result<()> {
    let hkey_current_user = RegKey::predef(HKEY_CURRENT_USER);
    let base_path = "Software\\Classes";
    
    // 1. Create a unique ProgID
    // Using the slug (e.g., "txt") ensures the registry key is "extexec_txt"
    let prog_id = format!("extexec_{}", ext.slug());
    let class_key_path = format!("{}\\{}", base_path, prog_id);
    
    println!("Creating Registry entry for ProgID: {}", prog_id);
    let (prog_key, _) = hkey_current_user.create_subkey(&class_key_path)?;
    prog_key.set_value("", &"File handled by extexec")?;

    // 2. Set the 'Open' command
    let cmd_path = format!("{}\\shell\\open\\command", class_key_path);
    let (cmd_key, _) = hkey_current_user.create_subkey(&cmd_path)?;
    
    let exe_str = exe_path.to_str().ok_or_else(|| {
        io::Error::new(io::ErrorKind::InvalidInput, "Path contains invalid UTF-8")
    })?;

    let command_value = format!("\"{}\" \"%1\"", exe_str);
    cmd_key.set_value("", &command_value)?;

    // 3. Link the extension to the ProgID
    let ext_key_path = format!("{}\\{}", base_path, ext.to_string());
    let (ext_key, _) = hkey_current_user.create_subkey(&ext_key_path)?;
    ext_key.set_value("", &prog_id)?;

    println!("Success: Associated {} with {}", ext, exe_str);
    Ok(())
}
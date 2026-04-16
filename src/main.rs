use clap::Parser;
use std::path::PathBuf;

mod extensions;
mod validator;

#[derive(Parser)]
#[command(
    name = "extexec",
    version,
    about = "A cross-platform tool to associate file extensions with executables",
    long_about = "extexec allows you to simulate the 'Open With' behavior by mapping file extensions to specific binaries given the file extension."
)]
struct Cli {
    /// The file extension to associate (e.g.: .txt, .md, ...)
    #[arg(short = 'e', long)]
    extension: extensions::Extension,

    /// The path to the executable to use
    #[arg(short = 'x', long, value_name = "FILE")]
    executable: PathBuf,

    /// Run the command without making any system changes
    #[arg(short = 'd', long)]
    dry_run: bool,
}

fn main() {
    let cli = Cli::parse();

    println!("--- extexec initialization ---");
    if let Err(e) = validator::validate_executable(&cli.executable) {
        eprintln!("Validation Error: {}", e);
        std::process::exit(1);
    }
    
    if cli.dry_run {
        println!("[DRY RUN] Would associate extension '{}' with: {:?}", cli.extension, cli.executable);
    } else {
        println!("Attempting to associate '{}' with: {:?}", cli.extension, cli.executable);
            
        todo!("Now I have to implement platform-specific association logics, starting from windows.");
    }
}

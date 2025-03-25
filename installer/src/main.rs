use log::{error, info};
use std::env;
use std::fs;
use std::path::Path;

fn init_logger() {
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("info")
    ).init();
}

fn main() {
    init_logger();

    let binary: &str = "harbour";
    let binary_name: String = Path::new(binary).file_name().unwrap().to_string_lossy().into_owned();
    let install_path: String = if cfg!(target_os = "windows") {
        let program_files: String = env::var("PROGRAMFILES").unwrap_or_else(|_| "C:\\Program Files".to_string());
        format!("{}\\{}", program_files, binary_name)
    } else {
        let user_path: String = env::var("HOME").unwrap_or_else(|_| "/usr/local/bin".to_string());
        format!("{}/.local/bin/{}", user_path, binary_name)
    };

    info!("Installing binary from: {}", binary);
    info!("Target install path: {}", install_path);

    if let Err(err) = fs::copy(binary, &install_path) {
        error!("Failed to copy binary: {}", err);
        return
    }

    info!("Installation complete! You can now run '{}'", binary_name);
}

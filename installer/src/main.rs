use clap::Parser;
use log::{error, info};
use std::env;
use std::fs;
use std::path::Path;

mod arguments;

fn init_logger() {
    env_logger::Builder::from_env(
        env_logger::Env::default()
            .default_filter_or("info")
    ).init();
}

fn main() {
    init_logger();

    let args = arguments::Arguments::parse();
    let binary_name = args.name.unwrap_or_else(|| Path::new(&args.binary).file_name().unwrap().to_string_lossy().into_owned());
    let install_path = if cfg!(target_os = "windows") {
        let program_files = env::var("PROGRAMFILES").unwrap_or_else(|_| "C:\\Program Files".to_string());
        format!("{}\\{}", program_files, binary_name)
    } else {
        let user_path = env::var("HOME").unwrap_or_else(|_| "/usr/local/bin".to_string());
        format!("{}/.local/bin/{}", user_path, binary_name)
    };

    info!("Installing binary from: {}", args.binary);
    info!("Target install path: {}", install_path);

    if let Err(err) = fs::copy(&args.binary, &install_path) {
        error!("Failed to copy binary: {}", err);
        return
    }

    info!("Installation complete! You can now run '{}'", binary_name);
}

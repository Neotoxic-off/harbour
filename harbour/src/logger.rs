use log::{info, warn, error};
use colored::*;

pub fn log_info(message: &str, value: Option<&str>) {
    match value {
        Some(v) => info!("{}: {}", message, v.green().bold()),
        None => info!("{}", message),
    }
}

pub fn log_warn(message: &str, value: Option<&str>) {
    match value {
        Some(v) => warn!("{}: {}", message, v.yellow().bold()),
        None => warn!("{}", message),
    }
}

pub fn log_error(message: &str, value: Option<&str>) {
    match value {
        Some(v) => error!("{}: {}", message, v.red().bold()),
        None => error!("{}", message.red().bold()),
    }
}

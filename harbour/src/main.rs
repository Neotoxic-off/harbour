mod models;
mod setup;
mod compose;
pub mod logger;

use models::arguments::Arguments;
use clap::Parser;
use logger::{log_error, log_info};

fn initlialize(args: Arguments) -> () {
    let mut generator: compose::Compose = compose::Compose::new(args);

    match generator.generate() {
        Ok(path) => log_info("Docker Compose file generated successfully at", Some(&path)),
        Err(e) => log_error("Failed to generate Docker Compose file", Some(&e.to_string()))
    }
}

fn main() {
    setup::init_logger();

    let args: Arguments = Arguments::parse();
    
    if !args.dockerfiles.is_empty() {
        initlialize(args);
        return;
    }

    log_error("No Dockerfiles provided!", None);
}

use log::{info, error};

mod models;
mod setup;
mod compose;

use models::arguments::Arguments;
use clap::Parser;

fn main() {
    setup::init_logger();

    let args: Arguments = Arguments::parse();
    
    if args.dockerfiles.is_empty() {
        error!("No Dockerfiles provided!");
        return;
    }

    let generator = compose::ComposeGenerator::new(args);
    match generator.generate() {
        Ok(path) => info!("Docker Compose file generated successfully at: {}", path),
        Err(e) => error!("Failed to generate Docker Compose file: {}", e)
    }
}

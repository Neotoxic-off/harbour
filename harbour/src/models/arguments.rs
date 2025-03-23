use clap::Parser;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Parser, Debug)]
#[command(
    version = "1.0", 
    author = "Harbour", 
    about = "🛟 Harbour is your rust powered port for container management"
)]
pub struct Arguments {
    #[arg(short, long, required = true, num_args = 1.., value_delimiter = ' ')]
    pub services: Vec<String>,

    #[arg(short, long)]
    pub network: Option<String>,

    #[arg(short, long, value_parser = clap::builder::PossibleValuesParser::new(
        ["no", "always", "on-failure", "unless-stopped"]
    ), default_value = "no")]
    pub restart: String,

    #[arg(short, long, default_value = "docker-compose.yml")]
    pub output: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub build: Option<String>,
    pub restart: Option<String>,
    pub networks: Option<Vec<String>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConfigFile {
    pub services: HashMap<String, ServiceConfig>,
    pub networks: Option<HashMap<String, NetworkConfig>>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub driver: Option<String>,
    pub external: Option<bool>,
    pub name: Option<String>
}

impl Arguments {
    
}

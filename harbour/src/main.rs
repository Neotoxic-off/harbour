use std::fs;
use std::path::Path;
use serde_yaml::{Value, Mapping};
use clap::Parser;
use log::{info, error};

mod models {
    pub mod arguments;
}

use models::arguments::Arguments;

fn setup() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
}

fn main() {
    setup();

    let args: Arguments = Arguments::parse();
    if !args.dockerfiles.is_empty() {
        let mut services = Mapping::new();

        for (i, file) in args.dockerfiles.iter().enumerate() {
            let service_name = Path::new(file)
                .parent()
                .and_then(|p| p.file_name())
                .unwrap_or_default()
                .to_str()
                .unwrap_or(&format!("service_{}", i))
                .to_string();

            info!("Adding service: {} from Dockerfile: {}", service_name, file);

            let mut service_config = Mapping::from_iter(vec![
                (Value::String("build".to_string()), Value::String(file.to_string())),
                (Value::String("restart".to_string()), Value::String(args.restart.clone()))
            ]);

            services.insert(
                Value::String(service_name),
                serde_yaml::to_value(service_config).unwrap()
            );
        }

        let mut compose = Mapping::from_iter(vec![
            (Value::String("services".to_string()), Value::Mapping(services))
        ]);

        let network_name = args.network.unwrap_or_else(|| "bridge".to_string());
        info!("Using network: {}", network_name);
        compose.insert(
            Value::String("networks".to_string()),
            serde_yaml::to_value(Mapping::from_iter(vec![
                (Value::String(network_name), serde_yaml::to_value(Mapping::from_iter(vec![
                    (Value::String("driver".to_string()), Value::String("bridge".to_string()))
                ])).unwrap())
            ])).unwrap()
        );

        let yaml_string = serde_yaml::to_string(&compose).expect("Failed to serialize yaml");
        fs::write("docker-compose.yml", yaml_string).expect("Unable to write file");
        info!("docker-compose.yml generated successfully!");
    } else {
        error!("No Dockerfiles provided!");
    }
}

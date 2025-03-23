use std::fs;
use std::path::Path;
use std::error::Error;
use serde_yaml::{Value, Mapping};
use log::{info, warn};

use crate::models::arguments::{Arguments, ServiceConfig};

pub struct ComposeGenerator {
    args: Arguments,
}

impl ComposeGenerator {
    pub fn new(args: Arguments) -> Self {
        Self { args }
    }

    pub fn generate(&self) -> Result<String, Box<dyn Error>> {
        let mut compose: Mapping = Mapping::new();

        let services: Mapping = self.generate_services()?;
        compose.insert(
            Value::String("services".to_string()),
            Value::Mapping(services)
        );

        let networks: Mapping = self.generate_networks()?;
        compose.insert(
            Value::String("networks".to_string()),
            Value::Mapping(networks)
        );

        let yaml_string = serde_yaml::to_string(&compose)?;
        fs::write(&self.args.output, yaml_string)?;
        
        Ok(self.args.output.clone())
    }
    
    fn generate_services(&self) -> Result<Mapping, Box<dyn Error>> {
        let mut services = Mapping::new();

        for (i, file) in self.args.dockerfiles.iter().enumerate() {
            let service_name = Path::new(file)
                .parent()
                .and_then(|p| p.file_name())
                .unwrap_or_default()
                .to_str()
                .unwrap_or(&format!("service_{}", i))
                .to_string();
                
            info!("Adding service: {} from Dockerfile: {}", service_name, file);

            let service_config: ServiceConfig = ServiceConfig {
                build: Some(file.to_string()),
                restart: Some(self.args.restart.clone()),
                networks: if let Some(network) = &self.args.network { 
                    Some(vec![network.clone()]) 
                } else { 
                    Some(vec![String::from("bridge")])
                }
            };

            if services.contains_key(&Value::String(service_name.clone())) {
                warn!("Service {} already defined in config file, skipping Dockerfile definition", service_name);
                continue;
            }
            
            services.insert(
                Value::String(service_name),
                serde_yaml::to_value(service_config)?
            );
        }
        
        Ok(services)
    }
    
    fn generate_networks(&self) -> Result<Mapping, Box<dyn Error>> {
        let mut networks = Mapping::new();

        if let Some(network_name) = &self.args.network {
            if !networks.contains_key(&Value::String(network_name.clone())) {
                info!("Adding network from CLI args: {}", network_name);

                let mut network_config = Mapping::new();
                network_config.insert(
                    Value::String("driver".to_string()),
                    Value::String("bridge".to_string())
                );
                
                networks.insert(
                    Value::String(network_name.clone()),
                    Value::Mapping(network_config)
                );
            }
        } else if networks.is_empty() {
            info!("Adding default bridge network");
            
            let mut network_config = Mapping::new();
            network_config.insert(
                Value::String("driver".to_string()),
                Value::String("bridge".to_string())
            );
            
            networks.insert(
                Value::String("bridge".to_string()),
                Value::Mapping(network_config)
            );
        }

        Ok(networks)
    }
}

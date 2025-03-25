use std::fs;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;
use serde_yaml::{Value, Mapping};
use serde::{Deserialize, Serialize};

use crate::logger::{log_info, log_warn, log_error};
use crate::models::arguments::Arguments;

#[derive(Debug, Serialize, Deserialize)]
pub struct ServiceConfig {
    pub build: Option<String>,
    pub volumes: Option<Vec<String>>,
    pub networks: Option<Vec<String>>,
    pub restart: Option<String>
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

pub struct Compose {
    args: Arguments
}

impl Compose {
    pub fn new(args: Arguments) -> Self {
        log_info("Initializing Compose with provided arguments", None);

        Self {
            args
        }
    }

    pub fn generate(&mut self) -> Result<String, Box<dyn Error>> {
        log_info("Starting compose generation", None);

        let mut compose: Mapping = self.load_existing_compose()?;
        let new_services: Mapping = self.generate_services()?;
        let new_networks: Mapping = self.generate_networks()?;

        self.merge_section(&mut compose, "services", new_services);
        self.merge_section(&mut compose, "networks", new_networks);

        self.render(&compose)
    }

    fn load_existing_compose(&self) -> Result<Mapping, Box<dyn Error>> {
        if Path::new(&self.args.output).exists() {
            log_info("Existing compose file found, loading", Some(&self.args.output));
            let content = fs::read_to_string(&self.args.output)?;
            match serde_yaml::from_str(&content) {
                Ok(Value::Mapping(existing)) => Ok(existing),
                Ok(_) => {
                    log_error("Invalid existing compose file structure", None);
                    Ok(Mapping::new())
                }
                Err(e) => {
                    log_error("Failed to parse existing compose file", Some(&e.to_string()));
                    Ok(Mapping::new())
                }
            }
        } else {
            log_info("No existing compose file found, starting fresh", None);
            Ok(Mapping::new())
        }
    }

    fn merge_section(&self, compose: &mut Mapping, section: &str, new_data: Mapping) {
        if let Some(Value::Mapping(existing_data)) = compose.get_mut(&Value::String(section.to_string())) {
            for (key, value) in new_data {
                if existing_data.contains_key(&key) {
                    log_warn(&format!("{} already defined in existing config", section), Some(&key.as_str().unwrap_or("unknown")));
                } else {
                    existing_data.insert(key, value);
                }
            }
        } else {
            compose.insert(Value::String(section.to_string()), Value::Mapping(new_data));
        }
    }

    fn render(&self, compose: &Mapping) -> Result<String, Box<dyn Error>> {
        log_info("Rendering the final output", None);

        let yaml_string = match serde_yaml::to_string(&compose) {
            Ok(yaml) => yaml,
            Err(e) => {
                log_error("Failed to serialize YAML", Some(&e.to_string()));
                return Err(Box::new(e));
            }
        };

        if let Err(e) = fs::write(&self.args.output, yaml_string) {
            log_error("Failed to write output file", Some(&e.to_string()));
            return Err(Box::new(e));
        }

        log_info("Output written to", Some(&self.args.output));

        Ok(self.args.output.clone())
    }

    fn generate_services(&self) -> Result<Mapping, Box<dyn Error>> {
        log_info("Generating services configuration", None);

        let mut services: Mapping = Mapping::new();

        if let Err(e) = self.process_service(&self.args.service, &mut services) {
            log_error("Failed to process service", Some(&e.to_string()));
        }

        Ok(services)
    }

    fn process_service(&self, service: &String, services: &mut Mapping) -> Result<(), Box<dyn Error>> {
        log_info("Processing Dockerfile for service", Some(&service));

        let mut service_config: ServiceConfig = self.create_service_config(service);

        if self.service_already_defined(services, &service) {
            log_warn("Service already defined, skipping", Some(&service));
            return Ok(());
        }

        service_config.volumes = match self.generate_volumes() {
            Ok(volumes) if !volumes.is_empty() => Some(volumes),
            Ok(_) => None,
            Err(e) => {
                log_error("Failed to generate volumes", Some(&e.to_string()));
                None
            }
        };

        services.insert(
            Value::String(service.clone()),
            serde_yaml::to_value(service_config).map_err(|e| {
                log_error("Failed to convert service config to YAML", Some(&e.to_string()));
                e
            })?
        );

        log_info("Service added successfully", Some(&service));

        Ok(())
    }

    fn create_service_config(&self, file: &String) -> ServiceConfig {
        log_info("Creating service configuration for file", Some(file));

        ServiceConfig {
            build: Some(file.to_string()),
            restart: Some(self.args.restart.clone()),
            volumes: None,
            networks: Some(vec![
                self.args.network.clone().unwrap_or_else(|| String::from("bridge"))
            ])
        }
    }

    fn service_already_defined(&self, services: &Mapping, service_name: &String) -> bool {
        let exists: bool = services.contains_key(&Value::String(service_name.clone()));

        if exists == true {
            log_warn("Service is already defined", Some(service_name));
        }

        exists
    }

    fn generate_networks(&self) -> Result<Mapping, Box<dyn Error>> {
        log_info("Generating networks configuration", None);
        let mut networks: Mapping = Mapping::new();
    
        match &self.args.network {
            Some(network_name) => self.add_custom_network(&mut networks, network_name),
            None if networks.is_empty() => self.add_default_network(&mut networks),
            _ => {}
        }
    
        Ok(networks)
    }

    fn generate_volumes(&self) -> Result<Vec<String>, Box<dyn Error>> {
        log_info("Generating volumes configuration", None);
        let mut volumes: Vec<String> = Vec::new();
    
        match &self.args.volume {
            Some(volume_value) => self.add_custom_volume(&mut volumes, volume_value),
            None => {}
        }
    
        Ok(volumes)
    }

    fn add_custom_network(&self, networks: &mut Mapping, network_name: &String) {
        log_info("Adding network", Some(network_name));

        if !networks.contains_key(&Value::String(network_name.clone())) {
            networks.insert(
                Value::String(network_name.clone()),
                Value::Mapping(self.create_bridge_config())
            );
            log_info("Network added successfully", Some(network_name));
        }
    }

    fn add_custom_volume(&self, volumes: &mut Vec<String>, volume: &String) {
        log_info("Adding volume", Some(volume));

        if volumes.contains(volume) == false {
            volumes.push(volume.clone());
            log_info("Volume added successfully", Some(volume));
        } else {
            log_warn("Volume already added", Some(volume));
        }
    }

    fn add_default_network(&self, networks: &mut Mapping) {
        log_info("Adding default bridge network", None);

        networks.insert(
            Value::String("bridge".to_string()),
            Value::Mapping(self.create_bridge_config())
        );

        log_info("Default bridge network added successfully", None);
    }

    fn create_bridge_config(&self) -> Mapping {
        log_info("Creating bridge network configuration", None);

        let mut network_config: Mapping = Mapping::new();

        network_config.insert(
            Value::String("driver".to_string()),
            Value::String("bridge".to_string())
        );

        network_config
    }
}

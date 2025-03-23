use std::fs;
use std::path::Path;
use std::error::Error;
use serde_yaml::{Value, Mapping};
use crate::logger::{log_info, log_warn, log_error};

use crate::models::arguments::{Arguments, ServiceConfig};

pub struct Compose {
    args: Arguments,
    services: Mapping,
    networks: Mapping
}

impl Compose {
    pub fn new(args: Arguments) -> Self {
        log_info("Initializing Compose with provided arguments", None);
        Self {
            args,
            services: Mapping::new(),
            networks: Mapping::new()
        }
    }

    pub fn generate(&mut self) -> Result<String, Box<dyn Error>> {
        log_info("Starting compose generation", None);
        let mut compose: Mapping = Mapping::new();

        self.services = self.generate_services()?;
        self.networks = self.generate_networks()?;

        compose.insert(
            Value::String("services".to_string()),
            Value::Mapping(self.services.clone())
        );
        compose.insert(
            Value::String("networks".to_string()),
            Value::Mapping(self.networks.clone())
        );

        self.render(&compose)
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

        for (i, file) in self.args.dockerfiles.iter().enumerate() {
            if let Err(e) = self.process_service(file, i, &mut services) {
                log_error("Failed to process service", Some(&e.to_string()));
            }
        }
        Ok(services)
    }

    fn process_service(&self, file: &String, index: usize, services: &mut Mapping) -> Result<(), Box<dyn Error>> {
        let service_name = self.get_service_name(file, index);

        log_info("Processing Dockerfile for service", Some(&service_name));

        let service_config = self.create_service_config(file);

        if self.service_already_defined(services, &service_name) {
            log_warn("Service already defined, skipping", Some(&service_name));
            return Ok(());
        }

        services.insert(
            Value::String(service_name.clone()),
            serde_yaml::to_value(service_config).map_err(|e| {
                log_error("Failed to convert service config to YAML", Some(&e.to_string()));
                e
            })?
        );
        log_info("Service added successfully", Some(&service_name));
        Ok(())
    }

    fn get_service_name(&self, file: &String, index: usize) -> String {
        log_info("Determining service name from file", Some(file));

        Path::new(file)
            .parent()
            .and_then(|p| p.file_name())
            .unwrap_or_default()
            .to_str()
            .unwrap_or(&format!("service_{}", index))
            .to_string()
    }

    fn create_service_config(&self, file: &String) -> ServiceConfig {
        log_info("Creating service configuration for file", Some(file));

        ServiceConfig {
            build: Some(file.to_string()),
            restart: Some(self.args.restart.clone()),
            networks: Some(vec![
                self.args.network.clone().unwrap_or_else(|| String::from("bridge"))
            ])
        }
    }

    fn service_already_defined(&self, services: &Mapping, service_name: &String) -> bool {
        let exists = services.contains_key(&Value::String(service_name.clone()));
        if exists {
            log_warn("Service is already defined", Some(service_name));
        }
        exists
    }

    fn generate_networks(&self) -> Result<Mapping, Box<dyn Error>> {
        log_info("Generating networks configuration", None);
        let mut networks: Mapping = Mapping::new();

        if let Some(network_name) = &self.args.network {
            self.add_custom_network(&mut networks, network_name);
        } else if networks.is_empty() {
            self.add_default_network(&mut networks);
        }

        Ok(networks)
    }

    fn add_custom_network(&self, networks: &mut Mapping, network_name: &String) {
        log_info("Adding custom network", Some(network_name));
        if !networks.contains_key(&Value::String(network_name.clone())) {
            networks.insert(
                Value::String(network_name.clone()),
                Value::Mapping(self.create_bridge_config())
            );
            log_info("Custom network added successfully", Some(network_name));
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

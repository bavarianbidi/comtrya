use super::Opt;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, path::PathBuf};

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Config {
    #[serde(default)]
    pub manifests: Vec<String>,

    pub variables: Option<BTreeMap<String, String>>,
}

/// Check the current working directory for a `Comtrya.yaml` file
/// If that doesn't exist, we'll check the platforms config directory
/// for comtrya/Comtrya.yaml
pub fn load_config(opts: Opt) -> Result<Config> {
    let mut config = match find_configs() {
        Some(file) => {
            let yaml = std::fs::read_to_string(file)
                .with_context(|| "Found Comtrya.yaml, but was unable to read the contents.")?;

            let mut config = match yaml.trim().is_empty() {
                true => Config {
                    ..Default::default()
                },

                false => serde_yaml::from_str(yaml.as_str())
                    .with_context(|| "Found Comtrya.yaml, but couldn't deserialize the YAML.")?,
            };

            // The existence of the config file allows an implicit manifests location of .
            if config.manifests.is_empty() {
                config.manifests.push(String::from("."));
            }

            config
        }

        None => Config {
            ..Default::default()
        },
    };

    if opts.manifest_location.is_some() {
        config.manifests = vec![opts.manifest_location.unwrap()];
    }

    Ok(config)
}

fn find_configs() -> Option<PathBuf> {
    // Check current working directory first
    if let Ok(cwd) = std::env::current_dir() {
        let local_config = cwd.join("Comtrya.yaml");

        if local_config.is_file() {
            return Some(local_config);
        }
    }

    // Check platform's config dir
    if let Some(config_dir) = dirs_next::config_dir() {
        let local_config = config_dir.join("Comtrya.yaml");

        if local_config.is_file() {
            return Some(local_config);
        }
    };

    None
}

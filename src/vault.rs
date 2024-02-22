use std::path::PathBuf;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::theme::NotedTheme;

#[derive(Debug, Deserialize, Serialize)]
pub struct VaultConfig {
    #[serde(default)]
    pub theme: NotedTheme
}

impl Default for VaultConfig {
    fn default() -> Self {
        VaultConfig {
            ..Default::default()
        }
    }
}

#[derive(Debug)]
pub struct Vault {
    base_path: PathBuf,
    pub(crate) vault_config: Mutex<VaultConfig>,
    vault_config_directory: PathBuf,
    vault_config_file: PathBuf,
}

impl Vault {
    pub fn new(base_path: PathBuf) -> Vault {
        // Init new vault here
        let vault_config_directory = base_path.join(".noted");

        if !vault_config_directory.exists() {
            let _ = std::fs::create_dir_all(&vault_config_directory);
        }

        let vault_config_file = vault_config_directory.join("config.ron");
        let vault_config = VaultConfig::default();

        if !vault_config_file.exists() {
            let data = ron::ser::to_string_pretty(&vault_config, ron::ser::PrettyConfig::default()).unwrap();
            let _ = std::fs::write(&vault_config_file, data);
        }

        let vault_config = Mutex::new(vault_config);

        let gitingore_file = vault_config_directory.join(".gitignore");
        if !gitingore_file.exists() {
            let data = include_str!("../data/vault_gitignore");
            let _ = std::fs::write(&gitingore_file, data);
        }

        Vault {
            base_path,
            vault_config,
            vault_config_directory,
            vault_config_file,
        }
    }

    pub fn parse_vault(base_path: PathBuf) -> Vault {
        // Parse vault here
        let vault_config_directory = base_path.join(".noted");
        let vault_config_file = vault_config_directory.join("config.ron");

        let vault_config = Mutex::new(if vault_config_file.exists() {
            let data = std::fs::read_to_string(&vault_config_file).unwrap();
            ron::de::from_str(&data).unwrap()
        } else {
            VaultConfig::default()
        });

        Vault {
            base_path,
            vault_config,
            vault_config_directory,
            vault_config_file,
        }
    }


    pub fn save_config(&self) {
        let data = ron::ser::to_string_pretty(&*self.vault_config.lock().unwrap(), ron::ser::PrettyConfig::default()).unwrap();
        let _ = std::fs::write(&self.vault_config_file, data);
    }
}

impl From<PathBuf> for Vault {
    fn from(base_path: PathBuf) -> Self {
        Vault::parse_vault(base_path)
    }
}
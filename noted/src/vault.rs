use std::{path::PathBuf, sync::Arc};
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use crate::theme::ConfigTheme;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VaultPath(String);

impl VaultPath {
    fn new(path: &str) -> VaultPath {
        VaultPath(path.to_string())
    }

    fn to_path_buf(&self, vault: &Vault) -> PathBuf {
        vault.base_path.join(&self.0)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct VaultConfig {
    #[serde(default)]
    pub theme: ConfigTheme,
    pub last_open_file: Option<VaultPath>
}

impl Default for VaultConfig {
    fn default() -> Self {
        VaultConfig {
            theme: ConfigTheme::default(),
            last_open_file: None
        }
    }
}

#[derive(Debug, Clone)]
pub struct Vault {
    base_path: PathBuf,
    pub(crate) vault_config: Arc<Mutex<VaultConfig>>,
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

        let vault_config = Arc::new(Mutex::new(vault_config));

        let gitingore_file = vault_config_directory.join("../../.gitignore");
        if !gitingore_file.exists() {
            let data = include_str!("../../data/vault_gitignore");
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

        let vault_config = Arc::new(Mutex::new(if vault_config_file.exists() {
            let data = std::fs::read_to_string(&vault_config_file).unwrap();
            ron::de::from_str(&data).unwrap()
        } else {
            VaultConfig::default()
        }));

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

    pub fn name(&self) -> String {
        self.base_path.file_name().unwrap().to_str().unwrap().to_string()
    }
}

impl From<PathBuf> for Vault {
    fn from(base_path: PathBuf) -> Self {
        Vault::parse_vault(base_path)
    }
}
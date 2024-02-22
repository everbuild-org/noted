use keyring::Entry;
use serde::Serialize;
use crate::vault::Vault;

#[derive(Debug, Serialize)]
pub enum SystemConfigKey {
    VaultBasePath,
}

impl SystemConfigKey {
    fn to_string(&self) -> String {
        match self {
            SystemConfigKey::VaultBasePath => "vault_base_path".to_string(),
        }
    }
}

pub fn read(key: SystemConfigKey) -> Option<String> {
    let name = key.to_string();
    let entry = Entry::new("noted", name.as_str());
    match entry {
        Ok(data) => {
            match data.get_password() {
                Ok(secret) => {
                    Some(secret)
                }
                Err(_) => None
            }
        }
        Err(_) => None
    }
}

pub fn write(key: SystemConfigKey, value: &str) -> Result<(), ()> {
    let name = key.to_string();
    let entry = Entry::new("noted", name.as_str());
    match entry {
        Ok(data) => {
            match data.set_password(value) {
                Ok(_) => Ok(()),
                Err(_) => Err(())
            }
        }
        Err(_) => Err(())
    }
}

pub fn init_system() -> Vault {
    let system_config = dirs::config_dir().unwrap().join("noted");
    let _ = std::fs::create_dir_all(&system_config);

    let default_vault = system_config.join("default_vault");
    let _ = std::fs::create_dir_all(&default_vault);

    // make sure there is a welcome.md file in the default vault
    let welcome_file = default_vault.join("welcome.md");
    if !welcome_file.exists() {
        let data = include_str!("../data/welcome.md");
        let _ = std::fs::write(welcome_file, data);
    }

    let base_path = read(SystemConfigKey::VaultBasePath);
    let vault = match base_path {
        Some(_) => {
            std::path::PathBuf::from(base_path.unwrap()).into()
        }
        None => {
            write(SystemConfigKey::VaultBasePath, default_vault.to_str().unwrap()).expect("Failed to write default vault path to system config");
            Vault::new(default_vault)
        }
    };
    vault.save_config();
    vault
}
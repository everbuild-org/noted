use crate::vault::Vault;

#[derive(Debug, Clone)]
pub enum RootMessage {
    LoadVault(Vault)
}
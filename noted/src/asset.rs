use std::borrow::Cow;
use anyhow::bail;
use gpui::{AssetSource, SharedString};
use crate::icon::asset_load_hook;

pub struct NotedAssetProvider;

impl AssetSource for NotedAssetProvider {
    fn load(&self, path: &str) -> anyhow::Result<Cow<'static, [u8]>> {
        if let Some(path) = asset_load_hook(path) {
            return path;
        }

        bail!("Unable to load asset: {}", path);
    }

    fn list(&self, path: &str) -> anyhow::Result<Vec<SharedString>> {
        return Ok(vec![]);
    }
}
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::app::{CollectionEntry, EstadoApp, HistoryEntry, SnapshotExport, data_dir};

fn history_path() -> PathBuf {
    data_dir().join("history.json")
}

fn collections_path() -> PathBuf {
    data_dir().join("collections.json")
}

fn env_path() -> PathBuf {
    data_dir().join("env.toml")
}

pub fn save_all(app: &EstadoApp) -> color_eyre::Result<()> {
    save_history(app)?;
    save_collections(app)?;
    save_env_vars(app)?;
    Ok(())
}

pub fn save_history(app: &EstadoApp) -> color_eyre::Result<()> {
    fs::create_dir_all(data_dir())?;
    let data = serde_json::to_string_pretty(&app.history)?;
    fs::write(history_path(), data)?;
    Ok(())
}

pub fn save_collections(app: &EstadoApp) -> color_eyre::Result<()> {
    fs::create_dir_all(data_dir())?;
    let data = serde_json::to_string_pretty(&app.collections)?;
    fs::write(collections_path(), data)?;
    Ok(())
}

pub fn save_env_vars(app: &EstadoApp) -> color_eyre::Result<()> {
    fs::create_dir_all(data_dir())?;
    let data = toml::to_string_pretty(&app.env_vars)?;
    fs::write(env_path(), data)?;
    Ok(())
}

pub fn exportar_snapshot(app: &EstadoApp, path: &Path) -> color_eyre::Result<()> {
    let data = serde_json::to_string_pretty(&app.snapshot())?;
    fs::write(path, data)?;
    Ok(())
}

pub fn importar_snapshot(path: &Path) -> color_eyre::Result<SnapshotExport> {
    let raw = fs::read_to_string(path)?;
    let parsed = serde_json::from_str(&raw)?;
    Ok(parsed)
}

pub fn load_history() -> color_eyre::Result<Vec<HistoryEntry>> {
    let path = history_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(path)?;
    let parsed = serde_json::from_str(&raw).unwrap_or_default();
    Ok(parsed)
}

pub fn load_collections() -> color_eyre::Result<Vec<CollectionEntry>> {
    let path = collections_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = fs::read_to_string(path)?;
    let parsed = serde_json::from_str(&raw).unwrap_or_default();
    Ok(parsed)
}

pub fn load_env_vars() -> color_eyre::Result<BTreeMap<String, String>> {
    let path = env_path();
    if !path.exists() {
        return Ok(BTreeMap::new());
    }
    let raw = fs::read_to_string(path)?;
    let parsed = toml::from_str(&raw).unwrap_or_default();
    Ok(parsed)
}

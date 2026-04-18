use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Manifest {
    pub appid: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub workshopid: Option<u64>,
    pub content: String,
    pub preview: String,
    pub title: String,
    pub description: String,
    pub visibility: u32,
    
    #[serde(skip)]
    source_path: Option<PathBuf>,
}

impl Manifest {
    /// List of default manifest filenames to check for when loading without a specified path.
    const MANIFEST_FILENAMES: [&'static str; 4] = [
        "manifest.json",
        "manifest.toml",
        "manifest.yaml",
        "manifest.yml",
    ];

    /// Load manifest from file, auto-detecting format (JSON, TOML, or YAML).
    /// If no format can be detected, an error is returned.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let content = fs::read_to_string(path)?;
        let path_str = path.to_string_lossy();

        let mut manifest: Self = if path_str.ends_with(".json") {
            serde_json::from_str(&content)?
        } else if path_str.ends_with(".toml") {
            toml::from_str(&content)?
        } else if path_str.ends_with(".yaml") || path_str.ends_with(".yml") {
            serde_yaml::from_str(&content)?
        } else {
            // throw an error
            return Err(format!("Could not detect manifest format from file extension for file: {}. Supported extensions are .json, .toml, .yaml, .yml", path_str).into());
        };

        manifest.source_path = Some(path.to_path_buf());
        Ok(manifest)
    }

    /// Load manifest from default locations: `manifest.json`, `manifest.toml`, `manifest.yaml`, or `manifest.yml`.
    pub fn load_default(manifest_path: Option<String>) -> Result<Self, Box<dyn std::error::Error>> {
        if let Some(path) = manifest_path {
            return Self::load(path);
        }
        for filename in &Self::MANIFEST_FILENAMES {
            if Path::new(filename).exists() {
                return Self::load(filename);
            }
        }
        Err("No manifest file found (manifest.json, manifest.toml, manifest.yaml, or manifest.yml)".into())
    }

    /// Save manifest to file, preserving or detecting format.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<(), Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let path_str = path.to_string_lossy();

        let content = if path_str.ends_with(".toml") {
            toml::to_string_pretty(self)?
        } else if path_str.ends_with(".yaml") || path_str.ends_with(".yml") {
            serde_yaml::to_string(self)?
        } else {
            // default to JSON
            serde_json::to_string_pretty(self)?
        };

        fs::write(path, content)?;
        Ok(())
    }

    /// Save manifest back to the original file it was loaded from.
    pub fn save_to_source(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(path) = &self.source_path {
            self.save(path)
        } else {
            Err("Manifest has no source path, cannot save".into())
        }
    }

    /// Save manifest with workshop ID updated to the original file it was loaded from.
    pub fn save_with_id_to_source(
        &mut self,
        workshopid: u64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.workshopid = Some(workshopid);
        self.save_to_source()
    }
}

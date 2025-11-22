use anyhow::{Ok, Result, anyhow};
use regex::Regex;
use serde::Deserialize;
use std::{collections::HashSet, path::Path, sync::Arc};

use crate::{domain::server::Server, infrastructure::state::AppState};

pub fn validate_name(input: &str) -> String {
    let re = Regex::new(r"[^A-Za-z0-9\-_]").unwrap();
    re.replace_all(input, "").to_string()
}

pub fn slugify(input: &str) -> String {
    let re = Regex::new(r"[^\w]+").unwrap();
    re.replace_all(input, "-").to_lowercase().to_string()
}

pub async fn validate_server(state: Arc<AppState>, server: &Server) -> Result<()> {
    if server.id < 1 {
        return Err(anyhow!("Invalid server id"));
    }

    let path = Path::new(&server.path);

    if !path.exists() {
        return Err(anyhow!(
            "Server directory does not exist for id: {}",
            server.id
        ));
    }

    if !path.is_dir() {
        return Err(anyhow!("Server directory is not a directory"));
    }

    let jar_path = path.join(&server.jar_path);

    if !jar_path.exists() {
        return Err(anyhow!(
            "Server does not contain a valid jar path for id: {}",
            server.id
        ));
    }
    validate_mc_type(&server.mc_type)?;

    state
        .version_manifest
        .write()
        .await
        .validate(&server.mc_version)
        .await?;
    validate_j_mem_arg(&server.j_max_mem)?;
    validate_j_mem_arg(&server.j_min_mem)?;

    Ok(())
}

pub fn validate_j_mem_arg(j_mem: &str) -> Result<()> {
    let re = Regex::new(r"(?i)^[0-9]+[KMG]$").unwrap();
    if re.is_match(j_mem) {
        Ok(())
    } else {
        Err(anyhow!("Invalid Java memory argument: {}", j_mem))
    }
}

pub fn validate_mc_type(input: &str) -> Result<()> {
    match input {
        "vanilla" | "paper" | "fabric" | "forge" => Ok(()),
        other => Err(anyhow!("Invalid Minecraft type: {}", other)),
    }
}

pub fn validate_mc_version(version: &str) -> Result<()> {
    let re = Regex::new(r"^(0|[1-9]\d*)\.(0|[1-9]\d*)(?:\.(0|[1-9]\d*))?$").expect("invalid regex");

    if !re.is_match(version) {
        return Err(anyhow!("Invalid Minecraft version format: {version}"));
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct MojangVersion {
    id: String,
    url: String,
}

#[derive(Debug, Deserialize)]
struct MojangVersionManifest {
    versions: Vec<MojangVersion>,
}

pub async fn load_valid_versions() -> Result<HashSet<String>> {
    let manifest = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
        .await?
        .error_for_status()?
        .json::<MojangVersionManifest>()
        .await?;

    Ok(manifest.versions.into_iter().map(|v| v.id).collect())
}

#[derive(Debug, Deserialize)]
struct MojangVersionDetails {
    downloads: MojangDownloads,
}

#[derive(Debug, Deserialize)]
struct MojangDownloads {
    server: Option<MojangDownloadArtifact>,
}

#[derive(Debug, Deserialize)]
struct MojangDownloadArtifact {
    url: String,
}

pub async fn get_vanilla_jar_url(version: &str) -> Result<String> {
    let manifest = reqwest::get("https://launchermeta.mojang.com/mc/game/version_manifest_v2.json")
        .await?
        .error_for_status()?
        .json::<MojangVersionManifest>()
        .await?;

    let entry = manifest
        .versions
        .into_iter()
        .find(|v| v.id == version)
        .ok_or_else(|| anyhow!("Version {} not found in manifest", version))?;

    let details = reqwest::get(&entry.url)
        .await?
        .error_for_status()?
        .json::<MojangVersionDetails>()
        .await?;

    let server = details
        .downloads
        .server
        .ok_or_else(|| anyhow!("No server download found for version {}", entry.id))?;

    Ok(server.url)
}

use std::{path::Path, sync::Arc};

use anyhow::{Result, anyhow};
use futures_util::StreamExt;
use reqwest::Client;
use std::fs;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, warn};
use uuid::Uuid;

use crate::domain::server::{self, NewServer, Server};
use crate::utils::validation::{get_vanilla_jar_url, validate_mc_type};
use crate::{domain::server::ServerCreator, infrastructure::state::AppState};

async fn download_jar(jar_url: &str, output_path: &str) -> Result<()> {
    let client = Client::new();

    let resp = client
        .get(jar_url)
        .send()
        .await
        .map_err(|e| anyhow!("Failed to GET {}: {}", jar_url, e))?;

    if !resp.status().is_success() {
        return Err(anyhow!("Bad response {} from {}", resp.status(), jar_url));
    }

    let file_path = Path::new(output_path).join("server.jar");
    let mut file = File::create(&file_path).await.map_err(|e| {
        anyhow!(
            "Failed to create file {}: {}",
            file_path.to_str().unwrap(),
            e
        )
    })?;

    let mut stream = resp.bytes_stream();

    while let Some(chunk) = stream.next().await {
        let data = chunk.map_err(|e| anyhow!("Failed to read chunk: {}", e))?;
        file.write_all(&data)
            .await
            .map_err(|e| anyhow!("Failed writing to {}: {}", output_path, e))?;
    }

    Ok(())
}

pub async fn create_new_server(
    state: Arc<AppState>,
    server_creator: ServerCreator,
) -> Result<Server> {
    server_creator.validate(state.clone()).await?;
    let new_uuid = Uuid::new_v4();
    let servers_path = Path::new(&state.config.master_path).join(&state.config.servers_path);
    if !servers_path.exists() {
        fs::create_dir_all(&servers_path)?;
    }
    let new_server_path = servers_path.join(new_uuid.to_string());

    fs::create_dir(&new_server_path)?;

    let jar_url = get_vanilla_jar_url(&server_creator.mc_version).await?;
    info!("{}", jar_url);

    download_jar(&jar_url, new_server_path.to_str().unwrap()).await?;

    let new_server = NewServer {
        name: server_creator.name,
        path: new_server_path.to_str().unwrap().to_string(),
        jar_path: "server.jar".to_string(),
        j_max_mem: server_creator.j_max_mem,
        j_min_mem: server_creator.j_min_mem,
        mc_type: server_creator.mc_type,
        mc_version: server_creator.mc_version,
    };

    let server = state.server_repo.create(new_server).await.map_err(|e| {
        error!(
            target = "api",
            error = ?e,
            "Failed to create server via /api/v1/server/add"
        );
        anyhow!("Failed to insert new server into database: {}", e)
    })?;

    let now = chrono::Local::now();
    let timestamp = now.format("%a %b %d %H:%M:%S %Z %Y").to_string();

    let eula_contents = format!(
        "#By changing the setting below to TRUE you are indicating your agreement to our EULA (https://aka.ms/MinecraftEULA).\n\
    #{}\n\
    eula=true\n",
        timestamp
    );

    let eula_path = new_server_path.join("eula.txt");
    fs::write(&eula_path, eula_contents)?;

    Ok(server)
}

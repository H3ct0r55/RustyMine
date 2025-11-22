use std::{collections::HashMap, sync::Arc};

use anyhow::Result;
use tokio::{
    sync::{RwLock, broadcast, mpsc},
    task::JoinHandle,
};

use crate::{
    domain::server::Server,
    infrastructure::{state::AppState, supervisor::run_server_worker},
    utils::validation::validate_server,
};

#[derive(Debug, Clone)]
pub enum ServerState {
    Stopped,
    Starting,
    Running,
    Stopping,
    Crashed(i32),
    Exited(i32),
}

#[derive(Debug, Clone)]
pub enum OutputSource {
    Stdout,
    Stderr,
}

#[derive(Debug, Clone)]
pub enum ServerEvent {
    StateChanged(ServerState),
    OutputLine { source: OutputSource, line: String },
}

#[derive(Debug)]
pub enum WorkerCommand {
    SendLine(String),
    Stop,
    Kill,
}

pub struct ManagedServer {
    pub id: i64,
    pub cmd_tx: mpsc::Sender<WorkerCommand>,
    pub events_tx: broadcast::Sender<ServerEvent>,
    pub _join: JoinHandle<()>,
}

#[derive(Clone)]
pub struct Supervisor {
    inner: Arc<RwLock<HashMap<i64, ManagedServer>>>,
}

impl Supervisor {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn start_server(&self, state: Arc<AppState>, server: Server) -> Result<()> {
        validate_server(state, &server).await?;
        let id = server.id;
        let (cmd_tx, cmd_rx) = mpsc::channel(32);
        let (events_tx, _) = broadcast::channel(256);

        let worker_server = server.clone();
        let worker_events_tx = events_tx.clone();

        let join = tokio::spawn(async move {
            if let Err(e) = run_server_worker(worker_server, cmd_rx, worker_events_tx).await {
                eprintln!("[{}] worker crashed: {e}", id);
            }
        });

        let managed = ManagedServer {
            id: server.id,
            cmd_tx,
            events_tx,
            _join: join,
        };

        let mut map = self.inner.write().await;
        map.insert(id, managed);
        Ok(())
    }

    pub async fn send_command(&self, id: i64, line: String) -> Result<()> {
        let map = self.inner.read().await;
        if let Some(s) = map.get(&id) {
            s.cmd_tx.send(WorkerCommand::SendLine(line)).await?;
        }
        Ok(())
    }

    pub async fn stop_server(&self, id: i64) -> Result<()> {
        let map = self.inner.read().await;
        if let Some(s) = map.get(&id) {
            s.cmd_tx.send(WorkerCommand::Stop).await?;
        }
        Ok(())
    }

    pub async fn subscribe(&self, id: i64) -> Option<broadcast::Receiver<ServerEvent>> {
        let map = self.inner.read().await;
        map.get(&id).map(|s| s.events_tx.subscribe())
    }
}

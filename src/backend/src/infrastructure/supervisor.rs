use std::process::Stdio;

use anyhow::Result;
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    process::Command,
    sync::{broadcast::Sender, mpsc::Receiver},
};

use crate::domain::{
    server::Server,
    supervisor::{OutputSource, ServerEvent, ServerState, WorkerCommand},
};

pub async fn run_server_worker(
    cfg: Server,
    mut cmd_rx: Receiver<WorkerCommand>,
    events_tx: Sender<ServerEvent>,
) -> Result<()> {
    let mut command = Command::new("java");

    command
        .current_dir(&cfg.path)
        .arg("-jar")
        .arg(&cfg.jar_path)
        .arg("nogui");

    command
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command.spawn()?;

    events_tx
        .send(ServerEvent::StateChanged(ServerState::Starting))
        .ok();

    let mut stdin = child.stdin.take().expect("child has no stdin");
    let stdout = child.stdout.take().expect("child has no stdout");
    let stderr = child.stderr.take().expect("child has no stderr");

    {
        let events_tx = events_tx.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let _ = events_tx.send(ServerEvent::OutputLine {
                    source: OutputSource::Stdout,
                    line,
                });
            }
        });
    }

    {
        let events_tx = events_tx.clone();
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                let _ = events_tx.send(ServerEvent::OutputLine {
                    source: OutputSource::Stderr,
                    line,
                });
            }
        });
    }

    events_tx
        .send(ServerEvent::StateChanged(ServerState::Running))
        .ok();

    loop {
        tokio::select! {
            maybe_cmd = cmd_rx.recv() => {
                match maybe_cmd {
                    Some(WorkerCommand::SendLine(line)) => {
                        if let Err(e) = stdin.write_all(line.as_bytes()).await {
                            eprintln!("[{}] failed to write to stdin: {e}", cfg.id);
                        }
                        if let Err(e) = stdin.write_all(b"\n").await {
                            eprintln!("[{}] failed to write newline: {e}", cfg.id);
                        }
                        let _ = stdin.flush().await;
                    }
                    Some(WorkerCommand::Stop) => {
                        let _ = stdin.write_all(b"stop\n").await;
                        let _ = stdin.flush().await;
                        events_tx.send(ServerEvent::StateChanged(ServerState::Stopping)).ok();
                    }
                    Some(WorkerCommand::Kill) => {
                        let _ = child.kill().await;
                        events_tx.send(ServerEvent::StateChanged(ServerState::Stopping)).ok();
                    }
                    None => {
                        let _ = child.kill().await;
                        break;
                    }
                }
            }

            exit = child.wait() => {
                match exit {
                    Ok(status) => {
                        let code = status.code().unwrap_or(-1);
                        let state = if status.success() {
                            ServerState::Exited(code)
                        } else {
                            ServerState::Crashed(code)
                        };
                        events_tx.send(ServerEvent::StateChanged(state)).ok();
                    }
                    Err(e) => {
                        eprintln!("[{}] child.wait() error: {e}", cfg.id);
                        events_tx.send(ServerEvent::StateChanged(ServerState::Crashed(-1))).ok();
                    }
                }
                break;
            }
        }
    }

    Ok(())
}

//GPT temporary code to test daemon
use anyhow::Result;
use std::io::{self, Read, Write};
use std::os::unix::net::UnixStream;

use crate::config::AppCfg;

pub async fn run_cli(_config: &AppCfg) -> Result<()> {
    let socket_path = &_config.sock_path;
    println!("RustyMine CLI (dev mode)");
    println!("Connected to daemon at: {socket_path}");
    println!("Type a message and press ENTER. Type 'quit' to exit.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut msg = String::new();
        io::stdin().read_line(&mut msg)?;

        let msg = msg.trim().to_string();

        if msg == "quit" {
            println!("Exiting CLI.");
            break;
        }

        match UnixStream::connect(socket_path) {
            Ok(mut stream) => {
                // Send to daemon
                if let Err(e) = stream.write_all(msg.as_bytes()) {
                    println!("Failed to send message: {e}");
                    continue;
                }

                // Receive response
                let mut buffer = [0u8; 1024];
                match stream.read(&mut buffer) {
                    Ok(n) if n > 0 => {
                        let resp = String::from_utf8_lossy(&buffer[..n]);
                        println!("Daemon replied: {resp}");
                    }
                    Ok(_) => {
                        println!("Received empty response from daemon.");
                    }
                    Err(e) => {
                        println!("Failed to read response: {e}");
                    }
                }
            }
            Err(e) => {
                println!("Failed to connect to daemon at {socket_path}: {e}");
            }
        }
    }

    Ok(())
}

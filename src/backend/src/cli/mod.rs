//GPT updated code to use JSON protocol
use anyhow::Result;
use serde_json::{Map, Value, json};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::os::unix::net::UnixStream;

use crate::config::AppCfg;
use crate::interface::protocol::{
    DaemonError, DaemonMessage, DaemonRequest, DaemonResponse, ResponseStatus,
};

pub async fn run_cli(config: &AppCfg) -> Result<()> {
    let socket_path = &config.sock_path;
    println!("RustyMine CLI (dev mode)");
    println!("Connected to daemon at: {socket_path}");
    println!("Type a command and press ENTER. Type 'quit' to exit.");
    println!("Examples:");
    println!("  ping");
    println!("  user.list");
    println!("  user.create --username steve --role admin\n");

    // Simple request counter for request_id
    let mut request_counter: u64 = 1;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut line = String::new();
        io::stdin().read_line(&mut line)?;

        let line = line.trim().to_string();
        if line.is_empty() {
            continue;
        }

        if line == "quit" {
            println!("Exiting CLI.");
            break;
        }

        // Parse "command --key value --key2 value2" into command + args JSON
        let (command, args) = match parse_command_line(&line) {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Parse error: {e}");
                continue;
            }
        };

        let req = DaemonRequest {
            protocol: 1,
            request_id: Some(request_counter.to_string()),
            command,
            args,
        };
        request_counter += 1;

        // Connect to daemon and send JSON
        match UnixStream::connect(socket_path) {
            Ok(stream) => {
                if let Err(e) = send_request_and_print_response(stream, &req) {
                    eprintln!("Request failed: {e}");
                }
            }
            Err(e) => {
                eprintln!("Failed to connect to daemon at {socket_path}: {e}");
            }
        }
    }

    Ok(())
}

/// Parse an input line like:
/// "user.create --username steve --role admin"
/// into ("user.create", { "username": "steve", "role": "admin" }).
fn parse_command_line(input: &str) -> Result<(String, Value), String> {
    let mut parts = input.split_whitespace();

    // First token = command
    let command = parts
        .next()
        .ok_or_else(|| "No command provided".to_string())?
        .to_string();

    let mut args_map = Map::new();
    let mut pending_key: Option<String> = None;

    for token in parts {
        if token.starts_with("--") {
            // We hit a new key; if previous key had no value, error.
            if pending_key.is_some() {
                return Err(format!(
                    "Missing value for argument --{}",
                    pending_key.unwrap()
                ));
            }
            let key = token.trim_start_matches("--").to_string();
            if key.is_empty() {
                return Err("Empty argument name after --".to_string());
            }
            pending_key = Some(key);
        } else {
            // This is a value for the last key
            if let Some(key) = pending_key.take() {
                args_map.insert(key, json!(token));
            } else {
                // Positional argument without a key: you can either
                //  - error out
                //  - or push to some "args" array
                // For now we error to keep things strict.
                return Err(format!(
                    "Unexpected positional argument '{token}' without a preceding --key"
                ));
            }
        }
    }

    if let Some(key) = pending_key {
        return Err(format!("Missing value for argument --{key}"));
    }

    Ok((command, Value::Object(args_map)))
}

/// Send a single request over the UnixStream, then read and print the JSON response.
fn send_request_and_print_response(mut stream: UnixStream, req: &DaemonRequest) -> Result<()> {
    // Serialize request as JSON + newline
    let json_req = serde_json::to_string(req)?;
    stream.write_all(json_req.as_bytes())?;
    stream.write_all(b"\n")?;
    stream.flush()?;

    // Read a single line as the response
    let mut reader = BufReader::new(stream);
    let mut resp_line = String::new();
    let n = reader.read_line(&mut resp_line)?;
    if n == 0 {
        eprintln!("Received empty response from daemon.");
        return Ok(());
    }

    let resp: DaemonResponse = serde_json::from_str(&resp_line)?;

    print_daemon_response(&resp);

    Ok(())
}

fn print_daemon_response(resp: &DaemonResponse) {
    match resp.status {
        ResponseStatus::Ok => {
            println!("Status: OK");
            if let Some(data) = &resp.data {
                if !data.is_null() {
                    println!(
                        "{}",
                        serde_json::to_string_pretty(data)
                            .unwrap_or_else(|_| "<invalid JSON>".into())
                    );
                }
            }
        }
        ResponseStatus::Error => {
            eprintln!("Status: ERROR");
            if let Some(err) = &resp.error {
                eprintln!("Code   : {}", err.code);
                eprintln!("Message: {}", err.message);
                if let Some(details) = &err.details {
                    eprintln!(
                        "Details: {}",
                        serde_json::to_string_pretty(details)
                            .unwrap_or_else(|_| "<invalid JSON>".into())
                    );
                }
            } else {
                eprintln!("(No error information provided by daemon)");
            }
        }
    }
}

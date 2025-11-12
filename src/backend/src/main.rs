use std::env;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    let child = Command::new("tree")
        .current_dir("/bin")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute child");

    let output = child.wait_with_output().expect("failed to wait on child");
    let text = String::from_utf8_lossy(&output.stdout);
    println!("Output: {}", text);
}

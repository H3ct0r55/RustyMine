// build.rs
use std::{env, fs, path::Path};

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let version_path = Path::new(&manifest_dir).join("version.toml");

    // Ensure build.rs reruns when version.toml changes
    println!("cargo:rerun-if-changed=version.toml");

    // If version.toml doesn't exist, create a sane default
    let default_contents = r#"channel = "d"
major = 0
minor = 1
build = 0
"#;

    let contents = fs::read_to_string(&version_path).unwrap_or_else(|_| {
        fs::write(&version_path, default_contents).expect("Failed to create default version.toml");
        default_contents.to_string()
    });

    let mut channel = String::from("d");
    let mut major = String::from("0");
    let mut minor = String::from("0");
    let mut build = String::from("0");

    for line in contents.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        if let Some((key, val)) = line.split_once('=') {
            let key = key.trim();
            let val = val.trim().trim_matches('"');

            match key {
                "channel" => channel = val.to_string(),
                "major" => major = val.to_string(),
                "minor" => minor = val.to_string(),
                "build" => build = val.to_string(),
                _ => {}
            }
        }
    }

    // Parse and increment build
    let mut build_num: u64 = build.parse().unwrap_or(0);
    build_num += 1; // 🔥 auto-increment

    // Write updated version.toml back to disk
    let new_contents = format!(
        "channel = \"{channel}\"\nmajor = {major}\nminor = {minor}\nbuild = {build}\n",
        channel = channel,
        major = major,
        minor = minor,
        build = build_num, // write incremented value
    );

    fs::write(&version_path, new_contents).expect("Failed to write updated version.toml");

    // Combined version string like d0.1.42
    let version_string = format!("{}{}.{}.{}", channel, major, minor, build_num);

    // Export env vars for use in Rust
    println!("cargo:rustc-env=VERSION_CHANNEL={channel}");
    println!("cargo:rustc-env=VERSION_MAJOR={major}");
    println!("cargo:rustc-env=VERSION_MINOR={minor}");
    println!("cargo:rustc-env=VERSION_BUILD={build_num}");
    println!("cargo:rustc-env=BUILD_VERSION={version_string}");
}

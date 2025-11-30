# RustyMine
A Rust-based Minecraft server manager.

## Table of Contents
- [Background](#background)
- [Install](#install)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [Contributing](#contributing)
- [License](#license)

## Background
After a forced restart corrupted my previous server manager, [Crafty 4](https://gitlab.com/crafty-controller/crafty-4), I started looking for alternatives that were:
- Robust against crashes  
- Purpose-built for Minecraft  
- Open source or source-available  

Nothing fit well, so I began building RustyMine — a clean, modern Minecraft server manager written entirely in Rust.  
This project will integrate with the upcoming supervisor, [MineGuard](https://github.com/H3ct0r55/MineGuard), which will handle process management, configuration, mod setup, and more.  
The project is still in early development and not yet usable, but active work is ongoing.

## Install
There are currently no official releases.  
If you'd like to experiment, you'll need to build the project from source.

## Usage
RustyMine uses:
- A REST API built with **Axum**
- A Web UI built with **React**

A CLI client that communicates with the REST API is planned, but not yet on the roadmap.

## Roadmap 
- [x] Basic project setup
- [ ] Backend user management
- [ ] Frontend skeleton with authentication
- [ ] Core [MineGuard](https://github.com/H3ct0r55/MineGuard) development
- [ ] MineGuard integration
- [ ] First public release

## Contributing
I am not accepting pull requests at this time.  
If you believe you can provide meaningful help, feel free to reach out directly:  
**h3cx@h3cx.dev**

## License
[PolyForm Noncommercial License 1.0.0](LICENSE)

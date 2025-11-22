# RustyMine -- Minecraft Server Manager

![Status](https://img.shields.io/badge/status-in_development-yellow)
![Backend](https://img.shields.io/badge/backend-Rust-orange)
![Frontend](https://img.shields.io/badge/frontend-React-blue)
![Runtime](https://img.shields.io/badge/runtime-Tokio-purple)
![Database](https://img.shields.io/badge/db-SQLite-lightgrey)
![License](https://img.shields.io/badge/license-PolyForm_Noncommercial-red)

RustyMine is a modern **Minecraft server management platform**, built
with a **Rust backend** and a **React management panel**.

The backend currently supports: - Adding Minecraft servers - Starting &
stopping servers - A work-in-progress Supervisor capable of launching
Java processes, reading STDOUT/STDERR, and sending STDIN commands - REST
API on port **8080** - SQLite-based persistence (path currently
hard‑coded in `config.rs`)


------------------------------------------------------------------------

## 🚧 Project Status

RustyMine is in **active early development**.\
The Supervisor works for Java processes but is far from complete.\
More API endpoints and full management features will come soon.

------------------------------------------------------------------------

## ⭐ Current Features

-   REST API (Axum)
-   Async process supervision using Tokio
-   Live log streaming (stdout & stderr)
-   Command piping to running Minecraft servers
-   SQLite storage for server metadata
-   Planned React control panel

------------------------------------------------------------------------

## 🔌 API v1 Endpoints

  Endpoint                      Method   Description
  ----------------------------- -------- -------------------
  `/api/v1/server/add`          `POST`   Add a new server
  `/api/v1/server/{id}`         `GET`    Fetch server info
  `/api/v1/server/{id}/start`   `POST`   Start a server

API base URL:\
**http://localhost:8080/api/v1**

------------------------------------------------------------------------

## 🧰 Setup

### **1. Clone the repository**

``` bash
git clone https://github.com/H3ct0r55/RustyMine.git
cd RustyMine/src/backend
```

### **2. Edit configuration**

RustyMine currently uses a **hard‑coded SQLite path** in:

    src/backend/src/config.rs

You **must** change this path before running RustyMine on your machine.

### **3. Run the backend**

``` bash
cargo run
```

------------------------------------------------------------------------

## 🗺 Roadmap (Short Term)

-   React management panel\
-   Proper configuration system (no more hard‑coded paths)\
-   Improved Supervisor lifecycle handling\
-   WebSocket console streaming\
-   Unified router for CLI + API\
-   Optional Java auto-install and environment detection

------------------------------------------------------------------------

## 📜 License

RustyMine is licensed under the: [PolyForm Noncommercial License 1.0.0](https://github.com/H3ct0r55/RustyMine/blob/main/LICENSE)

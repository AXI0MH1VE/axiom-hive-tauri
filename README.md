# Axiom Hive: Sovereign Desktop AI Engine

**Local-first AI desktop platform engineered for absolute privacy, provability, and sovereignty.**

---

## Overview

- **UI Container:** [Tauri v2 (Rust)](https://tauri.app/v2) — zero-bloat native container, RAM-efficient, strong sandboxing.
- **Core Logic:** Python scientific logic compiled as a [Nuitka](https://nuitka.net/pages/overview.html) binary, enabling both performance and IP protection.
- **Graph DB:** [KùzuDB](https://kuzudb.com) — on-disk, embedded graph database for relational/semantic memory.
- **Orchestration:** All queries decomposed and solved via Meta Deep Search (Tree-of-Thoughts, deterministic agentic workflow).
- **Branding:** "Hostile" visual palette (Absolute Black, Signal Red, Bone White), panther motif.
- **Sovereignty:** No user data ever leaves the local machine: all knowledge, logs, and configs live in a sandboxed %APPDATA%/AxiomHive or ~/.axiomhive folder.

---

## Architecture

![architext](docs/architext.svg)

### 1. Tauri v2 (Rust) — App Host

- Opens native window, handles system & hardware calls.
- Launches/monitors Python sidecar (compiled binary).
- Performs sovereign handshake: Cryptographically verifies Python logic binary before executing.
- Handles all I/O: file dialogs, notifications, tray, clipboard, drag/drop, etc.

### 2. Python Logic (Nuitka-compiled sidecar)

- **Algorithms:** Graph analytics (NetworkX), topology (TDA), deterministic retrieval, and synthesis.
- **Security:** Distributed as machine code, source code not exposed or interpretable post-compilation.
- **Interface:** Listens for stdin/stdout instructions from Rust (via pipes, not open sockets).

### 3. Embedded KùzuDB (Graph Database)

- All knowledge and indexes stored locally (user home or %APPDATA%).
- No vector DB SaaS, no external calls: sovereignty by design.
- Graph structure (nodes, edges, properties) supports Tree-of-Thoughts and provenance tracking.

### 4. Meta Deep Search Engine

- Uses ToT (Tree-of-Thoughts): decomposes queries, executes multi-branch searches in parallel, prunes low-confidence or low-source branches.
- Deterministic output synthesis, with traceable confidence and provenance markers for every statement.

---

## Build & Installation

### Prerequisites

- **Rust (latest stable)** — [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **Node.js >= 18** — [https://nodejs.org/](https://nodejs.org/)
- **Python 3.10+** (for development of sidecar)
- **Nuitka** — `pip install nuitka`
- **KùzuDB Python bindings** — `pip install kuzudb`
- **Tauri CLI** — `cargo install tauri-cli`
- **Additional Python packages:** `pip install networkx rich tomli`

### Build (Development)

1. **Compile Python Sidecar:**  
   ```sh
   cd axiom_hive/sidecar
   nuitka --standalone --onefile --enable-plugin=numpy --output-dir=dist main.py
   ```
   This creates an obfuscated, portable binary.

2. **Tauri App (Rust)**
   ```sh
   cd ..
   npm install
   tauri dev
   ```
   Electron is **not** used. No node server is bundled with production.

### Directories

- `src-tauri/` - Rust + Tauri system and IPC logic.
- `src/` - Frontend UI (Svelte or Yew).
- `sidecar/` - Python logic (only runs as compiled, never scripts).
- `db/` - Setup/init scripts for KùzuDB.
- `docs/` - Specifications, design, and references.

---

## Security Model

- **Sovereign Handshake:** Tauri verifies SHA-256 hash of Python sidecar on launch; no main logic is ever loaded if the binary hash doesn't match the trusted hash.
- **Data isolation:** All user data is stored in `%APPDATA%/AxiomHive` (Windows) or `~/.axiomhive/` (macOS/Linux). Nothing leaves device.
- **Bio-Hash Genesis:** Intended support for neural biometric registration (via [OpenBCI](https://openbci.com/) or similar) for genesis block creation (not yet in MVP).
- **No API keys, no phoning home, no analytics.** All computation/proof stays local.

---

## Visual Identity

- **Absolute Black** (`#000000`): Background, substrate.
- **Signal Red** (`#FF0022`): Active, warning, vector highlights.
- **Bone White** (`#F8F8F1`): Text, receipts, proofs.
- **Motif:** Panther, vectorized (SVG in `/public/panther.svg`).

---

## References & Sources

- [Tauri Getting Started](https://tauri.app/v2/guides/getting-started/)
- [Nuitka Binary Protection](https://nuitka.net/doc/user-manual.html#standalone)
- [KùzuDB Embedding](https://github.com/kuzudb/kuzu)
- [NetworkX Graph Algorithms](https://networkx.org/)
- [Tree-of-Thoughts (arXiv)](https://arxiv.org/abs/2305.10601)
- [Rich: Python Output Formatting](https://rich.readthedocs.io/en/stable/)
- [OpenBCI Bio-Hardware](https://openbci.com)
- [Axiom Hive v1 Whitepaper](docs/axiom-hive-whitepaper.md)

---

## Licensing

Licensed for private, personal, offline use only. Redistribution or modification for cloud/SaaS use is strictly forbidden.

```

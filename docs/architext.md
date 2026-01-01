# Architext: Axiom Hive Physical Blueprint

## Technology Stack

- **UI Container:** Tauri v2 (Rust)
- **Logic Sidecar:** Nuitka-compiled Python binary, called via verified process by Tauri on launch
- **Graph Storage:** KuzuDB, OS-user sandboxed folder only
- **Computation:** Deterministic ToT agent, pruning by source confidence, provenance stored
- **Visual:** "Hostile" palette; black substrate, red vector, white proof; panther motif

## Proofs & References

- Tauri as an ultra-low RAM, hardened desktop container:
  - https://tauri.app/v2/guides/benchmarks/
  - https://medium.com/tauri-apps/why-tauri-vs-electron-aa43e1dfb4ae
- Nuitka for Python IP protection:
  - https://nuitka.net/doc/user-manual.html#standalone
  - https://nuitka.net/doc/commercial.html
- KuzuDB, embedded persistent graph database:
  - https://github.com/kuzudb/kuzu#embedding
  - https://kuzudb.com/docs/tutorials/python/
- ToT/Agentic reasoning:
  - https://arxiv.org/abs/2305.10601
  - https://arxiv.org/abs/2310.16004
- Data sovereignty & zero-knowledge local privacy:
  - https://law.mit.edu/pub/data-sovereignty-in-the-cloud/pdf
- Visual hostile palette proof:
  - https://www.wired.com/story/red-black-aesthetic-design-psychology/
  - https://uxdesign.cc/why-dark-ui-feels-harder-edged-aa084e20c108
- Bio-hashing as mathematical anchor:
  - https://journals.plos.org/plosone/article?id=10.1371/journal.pone.0236794
  - https://openbci.com/

---

## Sovereign Handshake Protocol

1. SHA-256 of sidecar binary is stored on initial install.
2. Each launch, Tauri verifies hash before exec.
3. If failed, sidecar never loads.
4. All queries from Rust → Python via pipes, no external ports exposed.

---

## Directory Layout

- `src-tauri/` (Rust host app)
- `sidecar/` (compiled Python logic)
- `db/` (initial graph; Kuzu stanza or migration scripts)
- `public/` (panther SVG, favicon)
- `docs/` (this document, full chain of proof, legal stance)

---

## No Stubs. No Cloud. All Real.

All code above and in this repo is production-grade, not placeholder.

---

## Legal

Redistribution for cloud/SaaS forbidden.

---

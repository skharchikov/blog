---
order: 3
name: Blackjack (WIP)
slug: blackjack
description: Blackjack engine, server and TUI cli
github_url: https://github.com/skharchikov/blackjack
tags:
  - rust
  - axum
  - ratatui
---

## Event‑Driven CQRS Blackjack (Rust)

A modular **event‑sourced Blackjack engine** built as a **single‑writer state machine**.  
It consumes ordered **commands**, emits immutable **domain events**, and reconstructs state by replaying the event stream.  

The same core powers both a **TUI client** and a **WebSocket server**, while **CQRS read models** provide fast access to tables, snapshots, and leaderboards.

---

### Architecture

#### core
- Domain models and game rules  
- Command → Event decision logic  
- Pure event application (state = fold(events))  
- Deterministic sequencing per table  
- Designed for replay and auditability  

#### server
- Built with **Axum**
- WebSocket API for real‑time multiplayer interaction  
- Kafka-backed event streaming  
- Postgres projections for read models  

#### cli
- Terminal UI built with **ratatui + crossterm**
- Translates user input into commands  
- Renders state derived from projections  

---

### Tech Stack

- **Rust**
- **Tokio**
- **Axum** (HTTP + WebSocket)
- **Kafka** (event log / streaming backbone)
- **PostgreSQL** (snapshots & read models)
- **SQLx**
- **ratatui + crossterm**

---

### Key Concepts

- Event‑driven architecture  
- CQRS (separate write model and read projections)  
- Event sourcing  
- Monotonic sequencing per aggregate (table)  
- Modular Rust workspace (`core` / `server` / `cli`)  

# Rust Mastery — Drop-in Starter Repo

**Date:** 2025-09-12

## Quick start
```bash
git init
make setup            # installs pre-commit (fmt → clippy → tests)
cargo build           # build all workspace members
make demo             # run Axum gateway on :8080
make burst            # send sample orders to /orders
```

Then point your neo.mjs terminal to:
- WS: `ws://localhost:8080/ws/feed`
- HTTP: `http://localhost:8080/orders` and `/cancel`

## GitHub setup (once)
- Push the repo.
- Actions → run **Setup Kanban & Labels**.
- Actions → run **Setup Weekly Milestones** (enter Week 1 start date).
- Open a submission issue (e.g., `[lab01_enhanced_guessing_game] Submission: …`) — it will auto-label and appear on the **Course Kanban**.

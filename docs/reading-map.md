
# Appendix — Reading Map Across All 3 Stages

This appendix shows how the required texts, references, and Rust-specific resources are distributed across the entire program. Use it as a quick reference to know which chapters or sections to focus on each week.

---

## Stage 1 — Core Rust & Systems Foundations (Weeks 1–16)

**Primary Texts**
- *The Rust Programming Language* (a.k.a. Rust Book) — Chapters 1–12 (ownership, borrowing, error handling, traits, generics, modules).
- *Hands-On Rust* (Herbert Wolverson) — Chapters 1–10 for game dev foundations.
- *Rust in Action* (Tim McNamara) — Chapters 1–5 (systems programming, binary data, networking).
- *Rust for Rustaceans* (Jon Gjengset) — API design, idioms, modules.
- *Rust Atomics & Locks* (Mara Bos) — foundational concurrency concepts.
- *Effective Rust* (community draft) — style, API ergonomics, unsafe boundaries.

**By Week**
- Weeks 1–4 → Rust Book 1–9; *Hands-On Rust* 1–3; intro to Serde.
- Weeks 5–8 → *Hands-On Rust* 4–10; *Rust in Action* 1–5.
- Weeks 9–12 → *Rust in Action* 6–9; *Rust for Rustaceans* 1–2; *Rust Atomics & Locks* 1–2.
- Weeks 13–16 → *Rust Atomics & Locks* 3–8; *Rust for Rustaceans* 5–7; *Effective Rust* 1–6.

---

## Stage 2 — Quant Production Hardening (Weeks 17–34)

**Primary Texts**
- *Operating Systems: Three Easy Pieces (OSTEP)* — Processes, Scheduling, Concurrency, Memory, File Systems, I/O.
- *TCP/IP Illustrated Vol. 1* (Stevens) — Overview, Link Layer, IP, TCP basics, handshake, flow control, congestion, socket performance.
- *Inside the Machine* (Jon Stokes) — CPU pipeline, superscalar, OoO, branch prediction, cache hierarchy, memory ordering.

**By Week**
- Weeks 17–20 → OSTEP (Processes, I/O); TCP/IP (Overview, Link/IP, UDP vs TCP); *Inside the Machine* (CPU basics).
- Weeks 21–24 → OSTEP (Scheduling, I/O devices); TCP/IP (Handshake, Socket buffers); *Inside the Machine* (Caches).
- Weeks 25–28 → OSTEP (Threads, Concurrency); TCP/IP (Flow & Congestion); *Inside the Machine* (Superscalar/OoO).
- Weeks 29–32 → OSTEP (Malloc, Filesystems, Journaling); *Inside the Machine* (Branch prediction, Memory ordering).
- Weeks 33–34 → OSTEP (Crash consistency, Deadlock); TCP/IP (Keep-alives, retransmission); *Inside the Machine* (contention, false sharing).

---

## Stage 3 — Event-Driven Architecture & DDD (Weeks 35–52)

**Primary Texts**
- *Implementing DDD, CQRS and Event Sourcing* (Alex Lawrence) — Fundamentals of ES + CQRS.
- *Practical Microservices: Event-Driven Architectures* (Ethan Garofolo) — Microservices, messaging, sagas, observability.
- *Implementing Domain-Driven Design* (Vaughn Vernon) — Strategic and tactical DDD.
- EventModeling.org + Mermaid.js — Modeling workflows and narratives.

**By Week**
- Weeks 35–39 → Lawrence Ch. 1–6 (event sourcing basics, event stores, aggregates, read models, replay); Garofolo Ch. 1–5.
- Week 40 → Lawrence Ch. 7–8 (integration, testing).
- Weeks 41–45 → Vernon Ch. 7–10 (bounded contexts, services); Garofolo Ch. 6–10 (microservices, messaging, sagas, persistence, observability).
- Weeks 46–49 → Vernon Ch. 11–12 (event storming, collaborative modeling); EventModeling.org guides; Mermaid docs.
- Weeks 50–52 → Review & integration across all three texts; reflection and demo polish.

---

## Supplementary Rust Crates & Tools

- **Serialization/Persistence:** `serde`, `serde_json`, `toml`, `sled`, `sqlx`, `rusqlite`.  
- **Async & Concurrency:** `tokio`, `crossbeam`, `rayon`, `async-trait`.  
- **Error Handling:** `anyhow`, `thiserror`.  
- **Observability:** `tracing`, `metrics`, `prometheus`.  
- **Messaging:** `nats.rs`, `rdkafka`.  
- **Visualization:** `mermaid-cli`, EventModeling.org online tools.  
- **Ops/Infra:** Docker, docker-compose, GitHub Actions for CI/CD.

---

## How to Use This Reading Map

1. **Day 1 Focus:** Always start each week with readings (≈2h).  
2. **Cross-Linking:** Make explicit connections: e.g., OSTEP’s scheduling → Week 25 trading scheduler simulator.  
3. **Retention:** Use journals or flashcards for each reading section.  
4. **Application:** Every reading connects to that week’s build; treat the project as your *active note-taking*.  

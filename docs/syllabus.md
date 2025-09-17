
# Quant Systems & Architecture Degree (Rust-Focused)  

## Stage 1 — Core Rust & Systems Foundations (Weeks 1–16)

> **Pace:** 10h/week (2h/day × 5)  
> **Outcome:** A Rust-native **order book engine prototype** with deterministic replay, a CLI/TUI trading terminal, and foundational benchmarks.  
> **Texts Used This Stage:**  
> - *The Rust Programming Language* (“Rust Book”)  
> - *Hands-On Rust* (Wolverson)  
> - *Rust in Action* (McNamara)  
> - *Rust for Rustaceans* (Gjengset)  
> - *Rust Atomics & Locks* (Bos)  
> - *Effective Rust* (style/API guidelines)

---

## Week 1 — Lab 1: Enhanced Guessing Game (20 pts)

### Learning Objectives
- Cement ownership/borrowing, pattern matching, error handling.
- Build CLI UX and simple persistence.

### Background
- Ownership moves vs references; `Result<T, E>` and `match`; file I/O basics.

### Reading (Mon–Tue)
- Rust Book Ch. 1–4 (*Getting Started*, *Programming a Guessing Game*, *Common Concepts*).  
- Skim: Ch. 9 (*Error Handling*) for mindset.

### Requirements (Wed–Thu)
- Rebuild the classic guessing game; add:
  - Difficulty modes (easy/normal/hard → different ranges).
  - Persistent high scores (`std::fs`, store JSON/TOML with `serde`).
  - Replay loop with clean exit and error paths.
  - Configurable via flags (e.g., `--difficulty hard`) using `clap`.

### Testing & Docs (Fri)
- Unit tests for scoring logic & bounds checking.
- Document how scores are stored; show a sample score file and CLI examples.

### Stretch Goals
- Seeded RNG for reproducible runs.
- Colorized output; i18n strings table.

### Deliverables
- Executable (`cargo run -- --difficulty hard`).
- `README.md`, tests, sample score file.

### Rubric (20 pts)
- Functionality (game loop, replay, difficulty) — 6  
- Error handling (no panics; proper `Result` use) — 4  
- Persistence (correct read/write, robust to corrupt files) — 4  
- Code quality (layout, docs, clippy clean) — 6

---

## Week 2 — Lab 2: Config Parser CLI (20 pts)

### Learning Objectives
- Deserialize/serialize configs; practice modular design and CLI ergonomics.

### Background
- `serde`, `serde_json`/`toml`; keep lifetimes simple via owned configs or smart references.

### Reading (Mon–Tue)
- Rust Book Ch. 5–8 (Structs, Enums, Pattern Matching, Modules).  
- Serde quickstart (offline docs or examples).

### Requirements (Wed–Thu)
- `config edit --file settings.toml --set risk.max_order_size=1000`
- Support `read`, `set`, `delete`, `list`; round-trip preserving types.
- Defensive error messages (bad key paths, invalid types).

### Testing & Docs (Fri)
- Goldens: sample input files → expected output files.
- Unit tests for path parsing and round-trip.

### Stretch Goals
- Schema validation; default overlay (ENV > CLI > File).

### Deliverables
- `/config/cli/` crate, tests, sample configs, `README.md`.

### Rubric (20 pts)
- Parsing & round-trip — 6  
- CLI UX & helpful errors — 5  
- Design (modules, separation) — 4  
- Tests & docs — 5

---

## Week 3 — Lab 3: In-Memory KV Store (20 pts)

### Learning Objectives
- Practice collections, REPL loop, clean error handling, engine vs CLI separation.

### Background
- `HashMap`, `VecDeque`, ownership in collections; custom error types.

### Reading (Mon–Tue)
- Rust Book Ch. 9–12 (Error Handling, Generics, Traits, Lifetimes).

### Requirements (Wed–Thu)
- REPL: `set key val`, `get key`, `del key`, `list`.
- Optional persistence: snapshot to disk on exit.
- Clean separation: engine crate vs CLI binary.

### Testing & Docs (Fri)
- Integration tests: scripted sessions.
- (Optional) criterion microbench for `get/set`.

### Stretch Goals
- Append-only WAL (preview of durability).

### Deliverables
- `/kvstore/` crate, tests, `README.md`.

### Rubric (20 pts)
- Core operations — 6  
- REPL UX — 4  
- Modularity — 5  
- Tests/docs — 5

---

## Week 4 — Project 1: Game Extension (30 pts)

### Learning Objectives
- Architect a small system: entities, state machines, event loop; traits & enums.
- Deterministic replay mindset.

### Background
- Game loop anatomy; RNG seeding; trait-based entities.

### Reading (Mon–Tue)
- *Hands-On Rust* Ch. 1–3 (start here).
- Reference Rust Book chapters as needed.

### Requirements (Wed–Thu)
- Extend the starter game with:
  - Scoring + difficulty progression.
  - One substantial mechanic (AI enemy, power-ups, inventory, map).
  - Clean entity abstraction (trait-based) and tidy game loop.
  - Configurable controls; pause/resume; deterministic `--seed` option.

### Testing & Docs (Fri)
- Logic tests for your new mechanic.
- GIF/screenshot in README; design diagram (ASCII or image).

### Stretch Goals
- Save/load game state; high-score board.

### Deliverables
- `/game/` crate, tests, `README.md`, design diagram.

### Rubric (30 pts)
- Feature depth & integration — 10  
- Design clarity (traits, enums, modules) — 8  
- Robustness (errors, edge cases) — 6  
- Polish (docs, UX, tests) — 6

---

## Week 5 — Lab 4: Game Mechanic Deepening (25 pts)

### Learning Objectives
- Iterate on complexity; performance-aware updates; input/event handling.

### Background
- Time-step management; perf counters; profiling basics.

### Reading (Mon–Tue)
- *Hands-On Rust* Ch. 4–6.

### Requirements (Wed–Thu)
- Add two of: pathfinding, FOV/visibility, particle effects, status effects.
- Introduce time-step management (fixed or variable with interpolation).
- Frame budget logging (basic perf counters).

### Testing & Docs (Fri)
- Unit tests for new mechanics; document perf budget.

### Stretch Goals
- ECS-lite refactor (simple component/system split).

### Deliverables
- `/game/advanced/` crate, tests, `README.md`.

### Rubric (25 pts)
- Mechanic correctness — 8  
- Performance awareness — 5  
- Code organization — 6  
- Tests/docs — 6

---

## Week 6 — Lab 5: Binary File Inspector (20 pts)

### Learning Objectives
- Raw byte handling; slice safety; zero-copy where appropriate.

### Background
- `&[u8]` vs `Vec<u8>`; iterators; endian-aware parsing.

### Reading (Mon–Tue)
- *Rust in Action* Ch. 1–3 (I/O & bytes).

### Requirements (Wed–Thu)
- Hex dump with offsets + ASCII column.
- Optional pattern search and range views.
- Avoid unnecessary copies; discuss trade-offs in comments.

### Testing & Docs (Fri)
- Golden tests on fixtures; doc examples.

### Stretch Goals
- Pluggable parser for a tiny custom binary header.

### Deliverables
- `/tools/hexview/` crate, tests, `README.md`.

### Rubric (20 pts)
- Hex dump fidelity — 6  
- Search & range features — 6  
- Memory safety & efficiency — 4  
- Docs/tests — 4

---

## Week 7 — Lab 6: Concurrent HTTP Fetcher (25 pts)

### Learning Objectives
- Async concurrency with tokio; cancellation; timeouts; backoff; structured logs.

### Background
- Bounded concurrency patterns; `tracing` spans; retries.

### Reading (Mon–Tue)
- *Rust in Action* Ch. 4–5 (networking & concurrency intro).  
- Rust Book (async overview or appendix).

### Requirements (Wed–Thu)
- Read URLs from file; fetch concurrently; write bodies to disk.
- Timeouts, retry with exponential backoff, structured logs (`tracing`).
- Tunable concurrency (`--max-in-flight`).
- **Trading link:** optionally pull OHLCV from a public REST API and export CSV.

### Testing & Docs (Fri)
- Integration test hitting a local mock server; doc examples.

### Stretch Goals
- Rate limiting; per-host concurrency limits.

### Deliverables
- `/net/fetcher/` crate, tests, `README.md`.

### Rubric (25 pts)
- Correct async & concurrency limits — 10  
- Resilience (timeouts, retries) — 6  
- Observability (logs, metrics) — 4  
- Code quality/tests — 5

---

## Week 8 — Project 2: Second Game (30 pts)

### Learning Objectives
- Design a new small game emphasizing state management and data modeling.

### Background
- Data modeling via enums/traits; serialization hooks.

### Reading (Mon–Tue)
- *Hands-On Rust* Ch. 7–10 (finish).

### Requirements (Wed–Thu)
- New game (roguelike/puzzle/sim): clear loop, scoring, progression.
- Demonstrate pattern matching and composition via traits.
- Save/load state; deterministic seed for reproducible runs.

### Testing & Docs (Fri)
- Scenario tests; README with screenshots and controls.

### Stretch Goals
- Replay file (`.rpl`) and deterministic playback.

### Deliverables
- `/game2/` crate, tests, `README.md`.

### Rubric (30 pts)
- Game completeness & “fun factor” — 10  
- Idiomatic design & data modeling — 10  
- Reliability (save/load, tests) — 5  
- Presentation (README, screenshots) — 5

---

## Week 8.5 — Consolidation 1 (no points)

### Focus
- Revisit ownership & error handling across labs.
- Small refactors; remove unwraps; improve error messages.

### Output
- Short memo: “3 Rust pitfalls I solved and how.”

---

## Week 9 — Lab 7: Order State Machine (25 pts)

### Learning Objectives
- Formalize domain logic with enums and total pattern matching; trading order lifecycle.

### Background
- Transitions, invariants; error taxonomy with `thiserror`.

### Reading (Mon–Tue)
- *Rust in Action* Ch. 6–7 (state & data).  
- *Rust for Rustaceans* Ch. 1–2 (idioms, API design).

### Requirements (Wed–Thu)
- States: `New → Pending → PartiallyFilled → Filled | Cancelled`.
- Record reasons for transitions (e.g., cancel reason).
- Library + CLI; public API documented (docs + examples).

### Testing & Docs (Fri)
- Transition tests (no illegal edges); state diagram in docs.

### Stretch Goals
- IOC/FOK semantics as constraints on transitions.

### Deliverables
- `/orders/state/` crate, tests, `README.md`, rendered state diagram.

### Rubric (25 pts)
- Transition correctness — 10  
- API clarity (library design) — 6  
- Error modeling — 4  
- Docs/tests — 5

---

## Week 10 — Lab 8: Multi-Threaded Chat Server (30 pts)

### Learning Objectives
- Shared state with Arc, Mutex/RwLock; channels; graceful shutdown.

### Background
- Broadcast patterns; resource cleanup; backpressure basics.

### Reading (Mon–Tue)
- *Rust in Action* Ch. 8–9 (web, concurrency).  
- *Rust Atomics & Locks* Ch. 1–2 (threads, memory model).

### Requirements (Wed–Thu)
- TCP server; broadcast to all connected clients.
- Clean shutdown (Ctrl-C); handle disconnects.
- Compare two designs: Mutex-guarded vector vs channel fan-out (explain trade-offs).

### Testing & Docs (Fri)
- Integration tests (connect N clients, broadcast integrity).
- Architecture notes comparing designs.

### Stretch Goals
- Add per-client buffers and backpressure policy.

### Deliverables
- `/server/chat/` crate, tests, `README.md`, design comparison note.

### Rubric (30 pts)
- Concurrency correctness — 12  
- Resilience (disconnects, shutdown) — 6  
- Comparative design (two approaches explained) — 6  
- Code quality/tests — 6

---

## Week 11 — Lab 9: API Refactor (20 pts)

### Learning Objectives
- Library/bin split; public API ergonomics; configuration precedence; doc quality.

### Background
- Visibility; examples/doctests; error boundaries (`anyhow` for bin, `thiserror` for lib).

### Reading (Mon–Tue)
- *Rust for Rustaceans* Ch. 3–4 (modules, error handling).  
- *Effective Rust* Ch. 1–2 (style & API guidelines).

### Requirements (Wed–Thu)
- Extract a clean library; thin bin wrapper.
- Config file + CLI args + ENV merged into final config (document precedence).
- Public types & functions documented with examples; doctests run.

### Testing & Docs (Fri)
- Doc tests + integration tests; `cargo doc --open` quality pass.

### Stretch Goals
- Feature flags (e.g., `serde`/`rayon`) with clear API surface.

### Deliverables
- `/api/` lib crate + `/bin/` wrapper; tests; `README.md`.

### Rubric (20 pts)
- API design & docs — 8  
- Config system (precedence, validation) — 6  
- Tests (doc + integration) — 6

---

## Week 12 — Project 3: Mini Order Book (35 pts)

### Learning Objectives
- Data structures (e.g., `BTreeMap` for price levels); safe concurrency; benchmarking.

### Background
- Price level → FIFO queue; crossing/matching rules; partial fills.

### Reading (Mon–Tue)
- *Rust for Rustaceans* Ch. 5–6 (concurrency, unsafe boundaries).  
- *Rust Atomics & Locks* Ch. 3–4 (atomics, Mutex); revisit Ch. 1–2 as needed.

### Requirements (Wed–Thu)
- Maintain bids/asks with `price → VecDeque<Order>`.
- Support insert, cancel, match; generate trades/events.
- Concurrency model: choose **one** (single writer + multi-reader, sharded locks, or actor/channels); **justify it** in DESIGN.md.
- Benchmarks with criterion (throughput & latency under basic workloads).

### Testing & Docs (Fri)
- Unit + integration tests (edge cases: ties, cancels, partials).
- DESIGN.md with data structures, invariants, and concurrency rationale.

### Stretch Goals
- IOC/FOK order types; event log & simple replay CLI.

### Deliverables
- `/orderbook/mini/` crate, tests, `README.md`, `DESIGN.md`, `BENCH.md`.

### Rubric (35 pts)
- Correctness (matching rules, edge cases) — 12  
- Concurrency safety & justification — 10  
- Performance bench & analysis — 7  
- Code quality & docs — 6

---

## Week 13 — Lab 10: Lock-Free Queue (25 pts)

### Learning Objectives
- Understand atomics, ABA, memory ordering; compare lock-free vs lock-based.

### Background
- MPMC ring buffers; cache lines; false sharing.

### Reading (Mon–Tue)
- *Rust Atomics & Locks* Ch. 5–6 (lock-free data structures).  
- *Rust for Rustaceans* Ch. 7 (unsafe).

### Requirements (Wed–Thu)
- Implement or adapt an MPMC queue (ring with sequence numbers & padding).
- Safe external API; internal `unsafe` allowed if justified and documented.
- Bench vs `std::sync::mpsc` and `crossbeam::channel`.

### Testing & Docs (Fri)
- Stress tests (concurrent producers/consumers).
- Memory ordering notes (why `SeqCst`/`AcqRel` here).

### Stretch Goals
- Demonstrate reordering pitfall and fix with fences.

### Deliverables
- `/infra/lfq/` crate, tests, `README.md`, `BENCH.md`.

### Rubric (25 pts)
- Correctness under concurrency — 10  
- Memory model awareness (notes on ordering) — 5  
- Performance comparison (charts/tables) — 5  
- Docs/tests — 5

---

## Week 14 — Lab 11: Order Book Refactor with Effective Rust (20 pts)

### Learning Objectives
- Apply best practices: error taxonomies, API ergonomics, naming, modules.

### Background
- Reducing generic complexity; public API clarity; examples & doctests.

### Reading (Mon–Tue)
- *Effective Rust* Ch. 1–3.

### Requirements (Wed–Thu)
- Refactor Project 3 to improve:
  - Error types (`thiserror`), result propagation (`anyhow` for CLI).
  - Public API (fewer generics, clearer type aliases).
  - Module layout, visibility, and examples.
  - Add doctests; regenerate `cargo doc`.

### Testing & Docs (Fri)
- Diff review (before/after simplification); doc test pass.

### Deliverables
- Updated `/orderbook/mini/`, docs, doctests, CHANGELOG.

### Rubric (20 pts)
- API & error quality improvements — 8  
- Docs completeness (examples, doctests) — 6  
- Maintainability (diff shows simplification) — 6

---

## Week 15 — Lab 12: Benchmarking Locks vs Lock-Free (25 pts)

### **Learning Objectives**
- Rigorous benchmarking of synchronization primitives.
- Interpreting trade-offs between lock-based and lock-free structures.
- Avoiding common micro-benchmark pitfalls.

### **Background**
- Workload modeling; variance and confidence intervals.
- Scheduler noise and benchmarking hygiene.

### **Reading (Mon–Tue)**
- *Effective Rust* Ch. 4–6 (concurrency & unsafe boundaries).  
- *Rust Atomics & Locks* Ch. 7–8 (channels, RCU, synchronization patterns).

### **Requirements (Wed–Thu)**
- Implement two variants of a shared price-level structure:
  - **Mutex/RwLock version**.
  - **Lock-free/atomic version** (e.g., ring buffer or MPMC queue).
- Benchmark under workloads:
  - 90/10, 50/50, 10/90 read/write.
  - Single-producer vs multi-producer.
- Record results in `BENCH.md` with tables, plots, and analysis.

### **Testing & Docs (Fri)**
- Run benchmarks multiple times; show stability of results.
- Include raw CSV or JSON data for reproducibility.

### **Stretch Goals**
- Extend benchmark harness to simulate burst traffic like a market data feed.
- Introduce artificial delays to test fairness.

### **Deliverables**
- `/bench/locks_vs_lf/` crate, plots, `BENCH.md`, raw data.

### **Rubric (25 pts)**
- Experimental design (fair, repeatable) — **8**  
- Results clarity (tables/plots) — **7**  
- Analysis (when/why each wins) — **5**  
- Code quality — **5**

---

## Week 16 — Capstone A: Trading Terminal Prototype (50 pts)

### **Learning Objectives**
- Integrate all prior work into a cohesive trading system prototype.
- Build a deterministic, observable, and performant engine.
- Apply concurrency models safely and justify trade-offs.

### **Background**
- Order book and order state machine knowledge from Weeks 9–12.
- Concurrency tools from Weeks 13–15.
- Async/Tokio basics for feeds and I/O.

### **Reading (Mon–Tue)**
- Finish *Effective Rust*.  
- Spot-check *Rust for Rustaceans* chapters on library design & unsafe.  
- Revisit *Rust Atomics & Locks* sections relevant to your chosen concurrency model.

### **System Overview**
- **Core Engine:** Insert, cancel, match orders; maintain aggregated levels.  
- **Market Feed:** Async WebSocket-like simulator (CSV replay with Tokio).  
- **Backtester:** Deterministic replay to produce fills and P&L.  
- **CLI/TUI:** Start with a robust CLI; optional TUI using `ratatui` for live level-2 view.  
- **Observability:** `tracing` spans, structured logs, metrics; configurable via `config.toml`.

### **Minimum Requirements**
- Safe concurrency model with bounded backpressure.  
- Feed ingestion and burst handling.  
- Deterministic replay with latency and throughput metrics.  
- Well-structured, idiomatic Rust APIs with tests and documentation.

### **Stretch Goals**
- Multi-symbol support with snapshot + incremental updates.  
- IOC/FOK/iceberg orders; advanced matching rules.  
- WAL-based persistence and crash recovery.  
- Minimal TUI visualization of order book state.

### **Testing & Docs (Fri)**
- Integration tests across engine + feed + backtest.  
- Benchmarks (criterion) with p50/p95/p99 latencies.  
- Architecture diagram and decision log.  

### **Deliverables**
- `/capstoneA/` crate with:
  - `README.md`, `USAGE.md`, `DESIGN.md`, `BENCH.md`.
  - Architecture diagram & decision log.
  - Benchmarks and reproducible test data.

### **Rubric (50 pts)**
- Correctness & features (engine, feed, backtest) — **18**  
- Concurrency model (safety, backpressure, rationale) — **12**  
- Performance & evidence (benches, latencies, throughput) — **8**  
- API/Code quality (Effective Rust principles applied) — **6**  
- Docs/tests (guides, doctests, integration tests) — **6**

---

## Week 16.5 — Consolidation 2 (No Points)

### **Focus**
- Polish docs and codebase for Stage 1 completion.  
- Tag release **v1.0** for the trading engine prototype.  
- Ensure benchmarks are reproducible and deterministic replay works.  
- Verify `cargo test`, `cargo clippy`, and `cargo doc` are clean.  

### **Output**
- A clean, production-quality baseline to serve as the foundation for Stage 2 (**Quant Production Hardening**).


# Quant Systems & Architecture Degree (Rust-Focused)  

## Stage 2 — Quant Production Hardening (Weeks 17–34)

> **Pace:** 10h/week (2h/day × 5)  
> **Outcome:** A hardened, production-style Rust trading stack with observability, multi-feed fan-in, cache optimization, durability, risk checks, replication, and chaos resilience.  
> **Texts Used This Stage:**  
> - *Operating Systems: Three Easy Pieces* (OSTEP)  
> - *TCP/IP Illustrated, Vol. 1* (Stevens)  
> - *Inside the Machine* (Jon Stokes)  
> - *Rust Atomics & Locks* (Bos)  
> - *Rust for Rustaceans* (Gjengset)  
> - *Effective Rust* (API & systems design guidelines)

---

## Week 17 — Lab 13: Observability Baseline “Quant Top” (25 pts)

### Learning Objectives
- Introduce metrics and flamegraph tooling for trading engines.
- Develop awareness of tail latencies and system-level bottlenecks.

### Background
- Percentiles vs averages.  
- Exporter overhead and sampling strategies.  
- Flamegraphs for CPU hot path profiling.

### Reading (Mon–Tue)
- OSTEP: *Processes & the Process API*; Basic I/O & syscalls.  
- *Inside the Machine*: CPU tour; latency vs bandwidth mental model.

### Requirements (Wed–Thu)
- Add metrics: QPS, p50/p95/p99 order latency, GC/alloc counts, file descriptor count, RSS/CPU%.  
- `/metrics` text endpoint.  
- One-shot `flamegraph.sh` script.

### Testing & Docs (Fri)
- Metrics export in ≤5 ms per scrape.  
- Overhead <3% CPU at 10k ops/s.  
- Flamegraph highlights top 5% hot path.

### Stretch Goals
- Integrate with Prometheus text format.  
- Histogram-based latency tracking.

### Deliverables
- `/obs/metrics.rs`, `/obs/flamegraph.sh`, `README.md`, `EVAL.md`.

### Rubric (25 pts)
- Correctness — **10**  
- Measurements — **8**  
- Design — **5**  
- Code — **2**

---

## Week 18 — Lab 14: Market Data Sniffing & Decoder (30 pts)

### Learning Objectives
- Parse real-world trading protocols (FIX, ITCH, JSON).  
- Normalize market data into a unified event format.

### Background
- FIX tags and message types.  
- ITCH binary frame structure.  
- JSON feeds for simplified APIs.

### Reading (Mon–Tue)
- TCP/IP Illustrated Ch. 1–2 (link layer & IP).  
- OSTEP: I/O devices and file descriptors.  
- *Inside the Machine*: I/O paths overview.

### Requirements (Wed–Thu)
- Write a sniffer/decoder using pcap or raw sockets.  
- Normalize messages into internal `BookEvent` (add/reduce/cancel/trade).  
- Handle malformed frames gracefully.

### Testing & Docs (Fri)
- ≥99% valid parses on provided pcaps.  
- Median decode + normalize ≤30 µs on dev box.  
- Document schema mapping.

### Stretch Goals
- Add simple feed replayer using decoded events.  
- Multi-symbol support.

### Deliverables
- `/feeds/decoder/` crate, sample pcaps, `tests/decoder_cases.rs`.

### Rubric (30 pts)
- Correctness — **12**  
- Robustness — **6**  
- Measurements — **6**  
- Code/docs — **6**

---

## Week 19 — Lab 15: Syscall Cost Meter (25 pts)

### Learning Objectives
- Understand the syscall path from client to engine.  
- Measure syscall overhead and trade-offs.

### Background
- Context switches and traps.  
- Nagle’s algorithm and small writes.  
- Branch mispredict costs in pipelines.

### Reading (Mon–Tue)
- OSTEP: *Virtualization & traps; context switches*.  
- TCP/IP: *TCP handshake, states, timers*.  
- *Inside the Machine*: Pipeline basics.

### Requirements (Wed–Thu)
- Microbench: compare `write()`, `send()`, `sendmsg()` with different buffer sizes, Nagle on/off.  
- Use `strace` and `perf` to profile a single client submission.

### Testing & Docs (Fri)
- Show at least 10× latency difference across buffer sizes/Nagle toggles.  
- Explain top 3 syscalls in the critical path.

### Deliverables
- `/bench/syscall_cost/`, plots, `EVAL.md`.

### Rubric (25 pts)
- Measurements — **10**  
- Reasoning — **7**  
- Correctness — **5**  
- Code — **3**

---

## Week 20 — Lab 16: Multi-Feed Market Data Handler (30 pts)

### Learning Objectives
- Merge multiple exchange feeds into one coherent book.  
- Handle sequencing, out-of-order events, and hiccups.

### Background
- Socket buffers and flow control.  
- Scheduling preview; blocking vs non-blocking I/O.  
- Cache behavior on ingest path.

### Reading (Mon–Tue)
- TCP/IP: Socket buffers & flow control.  
- OSTEP: Scheduling intro.  
- *Inside the Machine*: Caches 101.

### Requirements (Wed–Thu)
- Multi-connection ingest (async or thread-per-connection).  
- Timestamp on arrival, normalize, sequence reconcile, publish to order book.  
- Recovery from dropped or delayed feeds.

### Testing & Docs (Fri)
- Drop rate 0 at 100k msgs/s on loopback.  
- Recovery in <1s from feed hiccup.  
- Document sequence reconciliation strategy.

### Deliverables
- `/feeds/fanin/`, `DESIGN.md`, soak test script.

### Rubric (30 pts)
- Correctness — **12**  
- Robustness — **7**  
- Design — **6**  
- Code/docs — **5**

---

## Week 21 — Lab 17: Scheduler Simulator (25 pts)

### Learning Objectives
- Explore scheduling policies for orders vs market data.  
- Quantify impact of scheduling choice on latency.

### Background
- Round-robin, shortest-job-first, priority, MLFQ.  
- Pipeline stalls and branch prediction.

### Reading (Mon–Tue)
- OSTEP: *CPU Scheduling*.  
- *Inside the Machine*: Branch prediction & pipeline stalls.

### Requirements (Wed–Thu)
- Simulate queues: `market_data`, `orders`, `persistence`.  
- Compare RR vs priority vs MLFQ.  
- Objective: minimize order-to-trade latency.

### Testing & Docs (Fri)
- Metrics reproducible within ±5%.  
- Policy choice justified with data in `EVAL.md`.

### Stretch Goals
- Add weighted fair queuing simulation.  
- Visualize queue length over time.

### Deliverables
- `/sim/scheduler/`, plots, `EVAL.md`.

### Rubric (25 pts)
- Measurements — **10**  
- Design — **8**  
- Correctness — **5**  
- Code/docs — **2**


## Week 22 — Capstone B: High-Throughput Feed Handler v1 (40 pts)

### Learning Objectives
- Build a production-grade fan-in powering the order book.
- Handle bursts, retries, and backpressure.

### Background
- Threads vs async tradeoffs.
- Queue bounding and retry policies.
- Superscalar/OoO execution and burst behavior.

### Reading (Mon–Tue)
- *Inside the Machine*: Superscalar, OoO overview.  
- OSTEP: *Threads* (intro), blocking vs async tradeoffs.

### Requirements (Wed–Thu)
- Harden Week 20 feed handler.
- Add bounded queues, backpressure, retries with jitter.
- Soak-test with bursty input.

### Testing & Docs (Fri)
- ≥300k msgs/s on loopback.  
- p95 update latency ≤2 ms.  
- Backpressure prevents OOM.  
- Report in 5-page design + failure modes.

### Deliverables
- `/feeds/fanin/v1/`, report, benchmarks.

### Rubric (40 pts)
- Correctness — **16**  
- Performance — **12**  
- Design — **8**  
- Code/docs — **4**

---

## Week 23 — Lab 18: Cache-Friendly Order Book Layout (30 pts)

### Learning Objectives
- Restructure order book for cache locality.
- Apply data-oriented design (AoS → SoA).

### Background
- Cache lines, associativity, prefetching.
- TLB basics.

### Reading (Mon–Tue)
- *Inside the Machine*: Cache hierarchy.  
- OSTEP: *Address spaces & paging*.

### Requirements (Wed–Thu)
- Refactor order book: SoA for hot paths.  
- Align structs to fit in 1–2 cache lines.  
- Avoid false sharing.

### Testing & Docs (Fri)
- ≥20% throughput gain or ≥30% p95 reduction.  
- Prove with flamegraphs + counters.

### Deliverables
- `/book/opt/cache_layout/`, microbenches, before/after plots.

### Rubric (30 pts)
- Measurements — **12**  
- Design — **9**  
- Correctness — **6**  
- Code/docs — **3**

---

## Week 24 — Lab 19: Lock-Free Ring Buffer (30 pts)

### Learning Objectives
- Replace locks on the hottest path.
- Practice MPMC design with memory ordering.

### Background
- Atomic fences, store/load buffers.
- Race conditions and synchronization.

### Reading (Mon–Tue)
- *Inside the Machine*: Memory ordering; store/load buffers.  
- OSTEP: *Concurrency & synchronization*.

### Requirements (Wed–Thu)
- Implement MPMC ring with seq numbers, wrap-around, padding.  
- Use between feed handler → matcher, and matcher → logger.  
- Demonstrate and fix reordering bug.

### Testing & Docs (Fri)
- Zero drops at 200k ev/s.  
- Latency stable vs mutex queue.  
- No deadlocks.  
- Invariants tested with Lamport checks.

### Deliverables
- `/infra/ringbuf/`, tests, `README.md`.

### Rubric (30 pts)
- Correctness — **12**  
- Robustness — **6**  
- Measurements — **7**  
- Code/docs — **5**

---

## Week 25 — Capstone C: Low-Latency Matching Engine (40 pts)

### Learning Objectives
- Turn order book into a quant-grade matching engine.
- Optimize allocations and branches.

### Background
- Object pools and allocators.
- Branch mispredict mitigation.

### Reading (Mon–Tue)
- OSTEP: *Malloc/allocators; heap behavior under contention*.  
- *Inside the Machine*: Branch mispredict mitigation.

### Requirements (Wed–Thu)
- Add order pools, remove hot-path allocations.  
- Reduce branches; precompute comparisons.  
- Benchmark p50/p95/p99.

### Testing & Docs (Fri)
- ≥2× throughput vs Week-17 baseline OR ≥40% p99 latency cut.  
- 8-hour soak: no corruption.  
- 10-page design doc.

### Deliverables
- `/engine/matcher/`, benchmarks, design doc.

### Rubric (40 pts)
- Performance — **14**  
- Correctness — **12**  
- Design — **8**  
- Code/docs — **6**

---

## Week 26 — Lab 20: Durable Logging (30 pts)

### Learning Objectives
- Ensure no committed trades are lost.
- Implement WAL and snapshots.

### Background
- Journaling and crash consistency.
- Reliability vs durability.

### Reading (Mon–Tue)
- OSTEP: *Filesystems, journaling, crash consistency*.  
- TCP/IP: *Reliability vs application-level durability*.

### Requirements (Wed–Thu)
- WAL with CRC32, append-only.  
- Periodic snapshot.  
- Crash-replay tool.

### Testing & Docs (Fri)
- Random kill test → replay restores all commits.  
- Snapshot restarts ≥3× faster.

### Deliverables
- `/persistence/wal/`, `wal_replay`, snapshotter, tests.

### Rubric (30 pts)
- Correctness — **12**  
- Robustness — **7**  
- Design — **6**  
- Code/docs — **5**

---

## Week 27 — Lab 21: Pre-Trade Risk Checks (30 pts)

### Learning Objectives
- Implement parallel, low-latency risk checks.
- Avoid stalling matching engine.

### Background
- Deadlock avoidance, sharded state.
- False sharing impact.

### Reading (Mon–Tue)
- OSTEP: *Locks, condition vars, deadlock avoidance*.  
- *Inside the Machine*: Contention and false sharing.

### Requirements (Wed–Thu)
- Checks: max order size, position, notional.  
- Lock-free or sharded state.  
- Async deny/allow.

### Testing & Docs (Fri)
- ≤100 µs p95 overhead.  
- Correct denial under race.  
- Restart → no lost/dup decisions.

### Deliverables
- `/risk/`, `DESIGN.md`, latency tests.

### Rubric (30 pts)
- Correctness — **12**  
- Robustness — **6**  
- Measurements — **6**  
- Design/docs — **6**

---

## Week 28 — Lab 22: Throughput/Latency Harness (30 pts)

### Learning Objectives
- Build a perf harness for end-to-end testing.
- Tune network settings.

### Background
- Congestion control, delayed ACKs, socket buffers.
- I/O scheduling basics.

### Reading (Mon–Tue)
- TCP/IP: *Congestion control, Nagle, QUICKACK*.  
- OSTEP: *I/O scheduling & performance*.

### Requirements (Wed–Thu)
- Load generator: open-loop + closed-loop.  
- Plot QPS vs latency.  
- Toggle Nagle, TCP_NODELAY, SO_SNDBUF/RCVBUF.

### Testing & Docs (Fri)
- Stable results across 5 runs (<10% stddev).  
- Show Nagle effect.  
- Recommend tuned defaults.

### Deliverables
- `/bench/quantperf/`, plots, tuning table, `EVAL.md`.

### Rubric (30 pts)
- Measurements — **12**  
- Correctness — **7**  
- Design — **6**  
- Code/docs — **5**

---

## Week 29 — Lab 23: Replication via Log Shipping (30 pts)

### Learning Objectives
- Add hot standby replication.
- Survive node loss without data loss.

### Background
- Keep-alives, retransmission, timeouts.
- IPC and fsync semantics.

### Reading (Mon–Tue)
- TCP/IP: *Keep-alives, retransmission, timeouts*.  
- OSTEP: *IPC & reliability patterns; fsync semantics*.

### Requirements (Wed–Thu)
- Stream WAL to follower; follower applies in order.  
- Add health/lag metrics.  
- Manual failover with leader flag.

### Testing & Docs (Fri)
- Kill primary under load → promotion catches up <5s.  
- WAL holes detected/repaired.

### Deliverables
- `/replication/logship/`, failover demo script.

### Rubric (30 pts)
- Correctness — **12**  
- Robustness — **7**  
- Design — **6**  
- Code/docs — **5**

---

## Week 30 — Lab 24: Chaos Harness (30 pts)

### Learning Objectives
- Prove resilience with fault injection.
- Define and enforce invariants.

### Background
- Crash consistency redux.  
- Memory ordering pitfalls across threads.

### Reading (Mon–Tue)
- OSTEP: *Crash consistency redux*.  
- *Inside the Machine*: Memory ordering pitfalls.

### Requirements (Wed–Thu)
- Kill processes, delay packets, corrupt WAL segment, drop follower.  
- Assertions on invariants: no duplicate fills, monotonic seq.

### Testing & Docs (Fri)
- 60-min chaos run with zero invariant violations.  
- Failures categorized and fixed.

### Deliverables
- `/chaos/`, invariants checklist, run logs.

### Rubric (30 pts)
- Robustness — **12**  
- Correctness — **9**  
- Design — **6**  
- Code/docs — **3**

---

## Week 31 — Lab 25: SLOs & Dashboards (25 pts)

### Learning Objectives
- Operate engine with real SLOs.
- Build dashboards and runbooks.

### Background
- RTT variance & jitter.  
- CPU counters for ops.

### Reading (Mon–Tue)
- TCP/IP: *RTT variance & jitter*.  
- *Inside the Machine*: CPU counters in ops.

### Requirements (Wed–Thu)
- Define SLOs (availability, p95 latency targets).  
- Metrics endpoint + dashboard.  
- Alerts + incident runbook.

### Testing & Docs (Fri)
- Alert fires when p95 > target 5 min.  
- Runbook resolves incident scenarios.

### Deliverables
- `/ops/`, dashboards (PNG), `RUNBOOK.md`, `SLOs.md`.

### Rubric (25 pts)
- Design — **10**  
- Correctness — **8**  
- Docs — **5**  
- Code — **2**

---

## Week 32 — Capstone D: Hardened Quant Stack (50 pts)

### Learning Objectives
- Integrate hardened components into a resilient stack.
- Operate under production-like conditions.

### Background
- Observability, feeds, matching, risk, persistence, replication, chaos.

### Reading (Mon–Tue)
- Review Stage 2 materials as needed.

### Requirements (Wed–Thu)
- Integrate: feed handler, matcher, risk, WAL, replication, ops.  
- Add `./scripts/bringup_cluster.sh` (1 primary + 1 follower).  
- Benchmarks: throughput, p50/p95/p99, recovery time, replication lag.

### Testing & Docs (Fri)
- Meets SLOs.  
- Kill primary mid-run → no lost commits; follower takeover <5s.  
- Snapshot restart ≥3× faster than WAL-only.

### Deliverables
- `/stack/` with cluster bring-up, benchmarks, dashboards.  
- 10–15 page technical paper: design, invariants, evaluation, failures.  
- Demo script.

### Rubric (50 pts)
- Correctness & fault tolerance — **15**  
- Performance & profiling — **10**  
- Design & invariants — **10**  
- Testing & CI — **5**  
- Observability & ops — **5**  
- Code/docs — **5**

---

## Week 33 — Consolidation 3 (No Points)

### Focus
- Review Stage 2 codebase.  
- Polish dashboards and ops docs.  
- Refactor brittle modules.  
- Archive benchmarks.

---

## Week 34 — Transition Week (No Points)

### Focus
- Write reflection memo:  
  - Top 3 performance bottlenecks solved.  
  - Biggest resilience lesson.  
  - 2 improvements left undone.  
- Prep for Stage 3 (**Event-Driven Architecture & DDD**).


# Quant Systems & Architecture Degree (Rust-Focused)  
## Stage 3 — Event-Driven Architecture & DDD (Weeks 35–52)

> **Pace:** 10h/week (2h/day × 5)  
> **Outcome:** A Rust-native distributed trading system built with event sourcing, CQRS, microservices, bounded contexts, and strategic DDD. Includes a final replayable simulator or audit pipeline with strong documentation and modeling.  
> **Texts Used This Stage:**  
> - *Implementing DDD, CQRS and Event Sourcing* (Alexey Zimarev / Alex Lawrence)  
> - *Practical Microservices: Event-Driven Architectures* (Ethan Garofolo)  
> - *Implementing Domain-Driven Design* (Vaughn Vernon)  
> - EventModeling.org guides + Mermaid.js  
> - Rust crates: `serde`, `tokio`, `tracing`, `nats`, `kafka`, `sled`, `sqlx`, `anyhow`, `thiserror`

---

## Week 35 — Lab 26: Event Sourcing Fundamentals (25 pts)

### Learning Objectives
- Understand event sourcing as an architectural style.
- Build a minimal event logger and player in Rust.

### Background
- Events as the source of truth.
- Append-only design and immutability.

### Reading (Mon–Tue)
- Lawrence: Ch. 1–2 (DDD & CQRS overview).  
- Garofolo: Ch. 1–2 (*Event-Driven Thinking*).  
- Rust Book refresher: modules & error handling.

### Requirements (Wed–Thu)
- CLI for logging domain events in memory.  
- Commands: `append`, `list`, `replay`.  
- Serialize events to JSON with `serde`.

### Testing & Docs (Fri)
- Unit tests for append and replay.  
- README with event sourcing primer.

### Stretch Goals
- Add timestamps and event IDs.  
- Deterministic replay from saved file.

### Deliverables
- `/events/logger/` crate, tests, `README.md`.

### Rubric (25 pts)
- Correctness — **10**  
- Design clarity — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 36 — Lab 27: Append-Only Event Store (25 pts)

### Learning Objectives
- Persist events to disk with Rust File I/O.
- Build foundation for replayable systems.

### Background
- File I/O modes and buffering.
- Serde with JSON/TOML.

### Reading (Mon–Tue)
- Lawrence: Ch. 3 (*Event Stores*).  
- Rust Book: Ch. 20 (*Final Project: Multithreaded Server*) for file patterns.

### Requirements (Wed–Thu)
- Implement file-backed event store.  
- CLI commands: `append`, `read`, `replay`.  
- Defensive error handling.

### Testing & Docs (Fri)
- Snapshot files vs golden tests.  
- Fault injection (corrupt event file).

### Stretch Goals
- Add compaction (merge snapshots).  
- Add `sled` backend option.

### Deliverables
- `/events/store/` crate, tests, sample files.

### Rubric (25 pts)
- Correctness — **10**  
- Persistence robustness — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 37 — Lab 28: Aggregates & Value Objects (25 pts)

### Learning Objectives
- Encapsulate domain logic in aggregates.
- Enforce invariants through value objects.

### Background
- Aggregate roots, commands, and invariants.
- Trade matching as a bounded aggregate.

### Reading (Mon–Tue)
- Lawrence: Ch. 4 (*Aggregates*).  
- Vernon: Ch. 5 (*Entities, Value Objects, Aggregates*).

### Requirements (Wed–Thu)
- Design domain logic for a basic matching engine.  
- Aggregate: OrderBook.  
- Value Objects: OrderId, Price, Quantity.

### Testing & Docs (Fri)
- Unit tests for invariant violations.  
- README with glossary of aggregates.

### Stretch Goals
- Add rejection events for invalid commands.

### Deliverables
- `/domain/aggregates/` crate, tests, glossary.

### Rubric (25 pts)
- Correctness — **10**  
- Domain model clarity — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 38 — Lab 29: Read Models & Projections (25 pts)

### Learning Objectives
- Separate write-side and read-side models.
- Implement fast queries with projections.

### Background
- CQRS principle: Command vs Query separation.
- Read models for dashboards and analytics.

### Reading (Mon–Tue)
- Lawrence: Ch. 5 (*Read Models*).  
- Garofolo: Ch. 4 (*CQRS Patterns*).

### Requirements (Wed–Thu)
- Add query layer with projections.  
- Store read models in HashMap or SQLite.  
- CLI: `query --by-order-id` or `query --all`.

### Testing & Docs (Fri)
- Integration tests for projections.  
- Doc: diagram showing read vs write.

### Stretch Goals
- Streaming updates with channels.

### Deliverables
- `/readmodels/` crate, tests, diagrams.

### Rubric (25 pts)
- Correctness — **10**  
- Efficiency — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 39 — Lab 30: Event Replay & Recovery (25 pts)

### Learning Objectives
- Rehydrate system state from events.
- Build resilience against crashes.

### Background
- Replay mechanics; testing with event streams.

### Reading (Mon–Tue)
- Lawrence: Ch. 6 (*Event Replay*).  
- Garofolo: Ch. 5 (*Fault Tolerance*).

### Requirements (Wed–Thu)
- Add replay command to rebuild state.  
- Simulate crash → reload state from events.  
- Validate integrity.

### Testing & Docs (Fri)
- Integration tests with simulated failures.  
- Doc: failure recovery scenario.

### Stretch Goals
- Checkpoint snapshots for faster recovery.

### Deliverables
- `/events/replay/`, tests, `EVAL.md`.

### Rubric (25 pts)
- Correctness — **10**  
- Recovery robustness — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 40 — Capstone E: Event-Sourced Trading CLI (40 pts)

### Learning Objectives
- Build a working trading CLI with event sourcing.
- Integrate aggregates, event store, projections.

### Background
- Commands → events → projections.
- Audit trail and determinism.

### Reading (Mon–Tue)
- Lawrence: Ch. 7–8 (*Integration & Testing*).

### Requirements (Wed–Thu)
- Trading domain: place order, cancel order, match trade.  
- Event store: append and replay.  
- Read models: query open orders, trades.  
- CLI/TUI: basic UX.

### Testing & Docs (Fri)
- Replay scenario from persisted log.  
- Audit log reproducible.

### Deliverables
- `/capstoneE/`, design doc, tests.

### Rubric (40 pts)
- Correctness — **15**  
- Domain coverage — **10**  
- Docs/tests — **8**  
- Code quality — **7**

---

## Week 41 — Lab 31: Microservice Design (25 pts)

### Learning Objectives
- Split system into bounded services.
- Define service boundaries.

### Background
- Matching, Risk, Strategy, Persistence.  
- Bounded context mapping.

### Reading (Mon–Tue)
- Vernon: Ch. 7 (*Bounded Contexts*).  
- Garofolo: Ch. 6 (*Microservice Principles*).

### Requirements (Wed–Thu)
- Separate services: OrderBook, Risk, Strategy.  
- Define RPC contracts (proto or JSON).  
- Run with cargo workspaces.

### Testing & Docs (Fri)
- Smoke tests for service boundaries.  
- Glossary of contexts.

### Deliverables
- `/services/`, glossary, configs.

### Rubric (25 pts)
- Correctness — **10**  
- Design clarity — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 42 — Lab 32: Messaging & Event Contracts (25 pts)

### Learning Objectives
- Add async messaging with NATS or Kafka.
- Formalize event contracts.

### Background
- Event schema evolution.  
- Broker guarantees (at-least-once vs exactly-once).

### Reading (Mon–Tue)
- Garofolo: Ch. 7 (*Messaging Systems*).  
- Lawrence: Ch. 9 (*Event Contracts*).

### Requirements (Wed–Thu)
- Implement async broker with `nats.rs` or Kafka.  
- Define schemas for OrderPlaced, TradeExecuted.  
- Add consumer + publisher crates.

### Testing & Docs (Fri)
- Integration tests with broker.  
- Contract versioning doc.

### Deliverables
- `/messaging/`, schemas, tests.

### Rubric (25 pts)
- Correctness — **10**  
- Robustness — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 43 — Lab 33: Saga & Workflow Orchestration (25 pts)

### Learning Objectives
- Manage long-running transactions with sagas.
- Model state machines for workflows.

### Background
- Saga pattern and compensating actions.
- Rust enums for state transitions.

### Reading (Mon–Tue)
- Garofolo: Ch. 8 (*Workflow Orchestration*).  
- Vernon: Ch. 10 (*Domain Services*).

### Requirements (Wed–Thu)
- Implement payment/order fill saga.  
- State transitions as Rust enums.  
- Compensation on failure.

### Testing & Docs (Fri)
- Integration tests for success/failure.  
- Saga diagram in docs.

### Deliverables
- `/workflow/saga/`, tests, docs.

### Rubric (25 pts)
- Correctness — **10**  
- Workflow clarity — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 44 — Lab 34: Polyglot Persistence (25 pts)

### Learning Objectives
- Use different stores for read vs write sides.
- Explore trade-offs of multiple databases.

### Background
- Write-heavy vs read-heavy stores.
- Postgres vs SQLite in Rust.

### Reading (Mon–Tue)
- Garofolo: Ch. 9 (*Polyglot Persistence*).  
- sqlx and rusqlite docs.

### Requirements (Wed–Thu)
- Write-side: tokio-postgres.  
- Read-side: rusqlite.  
- Abstract interface for persistence.

### Testing & Docs (Fri)
- Integration tests with both stores.  
- Doc explaining trade-offs.

### Deliverables
- `/persistence/polyglot/`, tests, docs.

### Rubric (25 pts)
- Correctness — **10**  
- Abstraction clarity — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 45 — Lab 35: Observability & Dockerization (25 pts)

### Learning Objectives
- Add tracing/logging to microservices.
- Package services with Docker.

### Background
- Distributed tracing basics.
- Dockerfiles for Rust services.

### Reading (Mon–Tue)
- Garofolo: Ch. 10 (*Observability*).  
- Docker + Rust guides.

### Requirements (Wed–Thu)
- Add `tracing` crate logs to services.  
- Dockerize each microservice.  
- Compose with docker-compose.

### Testing & Docs (Fri)
- Verify logs aggregated.  
- Container startup integration test.

### Deliverables
- `/infra/docker/`, Dockerfiles, compose file.

### Rubric (25 pts)
- Correctness — **10**  
- Observability — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 46 — Lab 36: Bounded Contexts & Ubiquitous Language (25 pts)

### Learning Objectives
- Define clear domain boundaries.
- Build a shared language for team design.

### Background
- Strategic DDD concepts.  
- Glossary-driven design.

### Reading (Mon–Tue)
- Vernon: Ch. 7–8 (*Bounded Contexts*).  
- EventModeling.org examples.

### Requirements (Wed–Thu)
- Write glossary of events, commands, aggregates.  
- Document bounded contexts.  
- Validate consistency.

### Testing & Docs (Fri)
- Peer review glossary.  
- Add to repo docs.

### Deliverables
- `/docs/domain/`, glossary, diagrams.

### Rubric (25 pts)
- Domain clarity — **10**  
- Completeness — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 47 — Lab 37: Context Maps & Integration (25 pts)

### Learning Objectives
- Map interactions between bounded contexts.
- Build integration adapters.

### Background
- FIX protocol as input.  
- Internal normalized events.

### Reading (Mon–Tue)
- Vernon: Ch. 9 (*Context Mapping*).  
- FIX protocol primers.

### Requirements (Wed–Thu)
- Adapter: FIX → Internal Event format.  
- Build translation layer crate.  
- Integration tests.

### Testing & Docs (Fri)
- Feed FIX logs → internal events.  
- Document mapping rules.

### Deliverables
- `/integration/fix_adapter/`, tests, docs.

### Rubric (25 pts)
- Correctness — **10**  
- Integration clarity — **7**  
- Docs/tests — **5**  
- Code quality — **3**

---

## Week 48 — Lab 38: Event Modeling Workshop (25 pts)

### Learning Objectives
- Visualize workflows with event modeling.
- Use diagrams as living architecture docs.

### Background
- Event storming/modeling methodology.
- Mermaid diagrams.

### Reading (Mon–Tue)
- EventModeling.org guide.  
- Vernon: Ch. 11 (*Event Storming*).

### Requirements (Wed–Thu)
- Model trade lifecycle with events, commands, views.  
- Diagram with Mermaid.  
- Add to docs.

### Testing & Docs (Fri)
- Validate model against running system.  
- README with diagrams.

### Deliverables
- `/docs/models/`, Mermaid files, README.

### Rubric (25 pts)
- Modeling clarity — **10**  
- Completeness — **7**  
- Docs — **5**  
- Quality — **3**

---

## Week 49 — Lab 39: Domain Storytelling & Team Design (25 pts)

### Learning Objectives
- Document flows with narrative form.
- Align technical + business perspectives.

### Background
- Domain storytelling practices.
- User stories as input.

### Reading (Mon–Tue)
- Vernon: Ch. 12 (*Collaborative Modeling*).

### Requirements (Wed–Thu)
- Write narrative flows for order lifecycle & risk.  
- Use diagrams to complement.  
- Align with bounded contexts.

### Testing & Docs (Fri)
- Peer review with stakeholders (or self-check vs glossary).  
- Integrate into docs.

### Deliverables
- `/docs/storytelling/`, stories, diagrams.

### Rubric (25 pts)
- Narrative clarity — **10**  
- Domain coverage — **7**  
- Docs — **5**  
- Quality — **3**

---

## Week 50 — Capstone F: Distributed Trading Simulator (50 pts)

### Learning Objectives
- Build a replayable distributed simulator.  
- Combine event sourcing, messaging, sagas, persistence.

### Background
- Integration of all Stage 3 components.  
- Replay as audit + backtest tool.

### Reading (Mon–Tue)
- Review Lawrence + Vernon integration chapters.

### Requirements (Wed–Thu)
- Replayable trading simulator OR real-time audit journal.  
- Multi-service architecture with NATS/Kafka.  
- Replay logs deterministically.  
- Collect metrics.

### Testing & Docs (Fri)
- End-to-end tests across services.  
- Docs: design, scenarios, evaluation.

### Deliverables
- `/capstoneF/`, code, 15–20 page tech summary, demo script.

### Rubric (50 pts)
- Correctness — **20**  
- Integration — **10**  
- Docs/tests — **10**  
- Code quality — **10**

---

## Week 51 — Consolidation 4 (No Points)

### Focus
- Review Stage 3 architecture.  
- Simplify brittle modules.  
- Ensure reproducible replays.  
- Update glossary and diagrams.

---

## Week 52 — Graduation Project & Reflection (No Points)

### Focus
- Final polish on simulator/audit pipeline.  
- Write reflection:  
  - Top 3 lessons about event-driven design.  
  - Hardest bug solved.  
  - What you’d improve with more time.  
- Prepare demo for portfolio/interviews.

# Rust Order-Book Engine + neo.mjs Terminal — One Pager
**What**: Low-latency Rust engine driving a live neo.mjs terminal.  
**Why**: Correctness, concurrency, performance (p95/p99), ops polish.  
**How**: Single-writer + MPMC, snapshot+delta, idempotent orders, reproducible backtests.

# Live Demo Script (10 minutes)
1) `make demo` — start gateway on :8080
2) Start neo.mjs UI, subscribe to DEMO: snapshot → deltas render
3) `make burst` — hit POST /orders; show `/metrics`
4) Kill & restart gateway; watch UI resubscribe and resync snapshot

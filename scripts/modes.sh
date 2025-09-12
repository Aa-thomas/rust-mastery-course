#!/usr/bin/env bash
cat <<'TABLE'
+-------------------+-----------+-----------+-----------+
| Mode              | Throughput|   p95 (ms)|   p99 (ms)|
+-------------------+-----------+-----------+-----------+
| Mutex (batched)   |   120k/s  |      3.5  |      7.9  |
| Actors (channels) |   150k/s  |      2.7  |      6.1  |
| Lock-free (MPMC)  |   180k/s  |      2.1  |      4.3  |
+-------------------+-----------+-----------+-----------+
TABLE
echo "(Replace with your real results later.)"

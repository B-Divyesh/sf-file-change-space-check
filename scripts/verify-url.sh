#!/usr/bin/env bash
set -euo pipefail

URL=${1:?usage: scripts/verify-url.sh URL EVIDENCE_DIRECTORY}
EVIDENCE_DIRECTORY=${2:?usage: scripts/verify-url.sh URL EVIDENCE_DIRECTORY}
mkdir -p "$EVIDENCE_DIRECTORY"
STATUS=$(curl -sS -o "$EVIDENCE_DIRECTORY/index.html" -w "%{http_code}" --max-time 30 "$URL")
printf 'GET %s -> %s\n' "$URL" "$STATUS"
test "$STATUS" = "200"
grep -qi '<html' "$EVIDENCE_DIRECTORY/index.html"
node scripts/verify-url.mjs "$URL" "$EVIDENCE_DIRECTORY"

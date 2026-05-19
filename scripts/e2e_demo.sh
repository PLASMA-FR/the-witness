#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="$ROOT/target/release/the-witness"
TMP_DIR="$(mktemp -d "${TMPDIR:-/tmp}/witness-e2e.XXXXXX")"
CONFIG="$TMP_DIR/witness.toml"
if [ -n "${WITNESS_E2E_UPSTREAM_PORT:-}" ] && [ -n "${WITNESS_E2E_PROXY_PORT:-}" ]; then
  if ! [[ "$WITNESS_E2E_UPSTREAM_PORT" =~ ^[0-9]+$ && "$WITNESS_E2E_PROXY_PORT" =~ ^[0-9]+$ ]]; then
    echo "WITNESS_E2E_UPSTREAM_PORT and WITNESS_E2E_PROXY_PORT must be numeric" >&2
    exit 2
  fi
  UPSTREAM_PORT="$WITNESS_E2E_UPSTREAM_PORT"
  PROXY_PORT="$WITNESS_E2E_PROXY_PORT"
else
  read -r UPSTREAM_PORT PROXY_PORT < <(python3 - <<'PY'
import socket
ports = []
for _ in range(2):
    s = socket.socket()
    s.bind(('127.0.0.1', 0))
    ports.append(s.getsockname()[1])
    s.close()
print(*ports)
PY
)
fi
UPSTREAM_LOG="$TMP_DIR/upstream.log"
PROXY_LOG="$TMP_DIR/proxy.log"
UPSTREAM_PID=""
PROXY_PID=""

cleanup() {
  if [ -n "$PROXY_PID" ] && kill -0 "$PROXY_PID" 2>/dev/null; then kill "$PROXY_PID" 2>/dev/null || true; fi
  if [ -n "$UPSTREAM_PID" ] && kill -0 "$UPSTREAM_PID" 2>/dev/null; then kill "$UPSTREAM_PID" 2>/dev/null || true; fi
  rm -rf "$TMP_DIR"
}
trap cleanup EXIT

if [ ! -x "$BIN" ]; then
  echo "release binary missing; building first"
  cargo build --release --manifest-path "$ROOT/Cargo.toml"
fi

cat >"$CONFIG" <<EOF
[gemma]
backend = "demo"
model = "demo-judge"
url = "http://localhost:11434"
setup_completed = true

[setup]
last_doctor_check = ""
judge_schema_test_passed = true
proxy_test_passed = true
model_test_passed = true
demo_mode = true

[defaults]
retry_limit = 2
strictness = "medium"
fallback_mode = "safe_response"
log_format = "jsonl"
privacy_mode = false

[[endpoints]]
name = "Demo"
enabled = true
upstream_url = "http://127.0.0.1:$UPSTREAM_PORT"
local_proxy_url = "http://127.0.0.1:$PROXY_PORT/Demo/v1"
model = "mock-upstream"
profile = "education"
retry_limit = 2
strictness = "medium"
fallback_mode = "safe_response"
auth_header = ""
timeout_seconds = 10
EOF

python3 - "$UPSTREAM_PORT" <<'PY' >"$UPSTREAM_LOG" 2>&1 &
import json, sys
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer

port = int(sys.argv[1])
state = {"count": 0}

class Handler(BaseHTTPRequestHandler):
    def log_message(self, fmt, *args):
        return
    def do_POST(self):
        length = int(self.headers.get('content-length', '0'))
        self.rfile.read(length)
        state['count'] += 1
        if state['count'] == 1:
            content = '2 + 2 equals 5 because numbers are flexible.'
        else:
            content = '2 + 2 = 4 because adding two items to two more items gives four items.'
        body = {
            'id': f'mock-{state["count"]}',
            'object': 'chat.completion',
            'choices': [{'message': {'role': 'assistant', 'content': content}}],
        }
        encoded = json.dumps(body).encode()
        self.send_response(200)
        self.send_header('content-type', 'application/json')
        self.send_header('content-length', str(len(encoded)))
        self.end_headers()
        self.wfile.write(encoded)

ThreadingHTTPServer(('127.0.0.1', port), Handler).serve_forever()
PY
UPSTREAM_PID=$!

for _ in $(seq 1 50); do
  if python3 - "$UPSTREAM_PORT" <<'PY' >/dev/null 2>&1
import socket, sys
s = socket.create_connection(('127.0.0.1', int(sys.argv[1])), timeout=0.2)
s.close()
PY
  then break; fi
  sleep 0.1
done

"$BIN" --config "$CONFIG" start --proxy-addr "127.0.0.1:$PROXY_PORT" >"$PROXY_LOG" 2>&1 &
PROXY_PID=$!
for _ in $(seq 1 50); do
  if python3 - "$PROXY_PORT" <<'PY' >/dev/null 2>&1
import socket, sys
s = socket.create_connection(('127.0.0.1', int(sys.argv[1])), timeout=0.2)
s.close()
PY
  then break; fi
  sleep 0.1
done

RESPONSE="$TMP_DIR/response.json"
curl -fsS \
  -H 'content-type: application/json' \
  -d '{"model":"mock-upstream","messages":[{"role":"user","content":"Explain why 2 + 2 = 4 in one sentence."}]}' \
  "http://127.0.0.1:$PROXY_PORT/Demo/v1/chat/completions" >"$RESPONSE"

python3 - "$RESPONSE" "$TMP_DIR/logs/witness.jsonl" <<'PY'
import json, pathlib, sys
response = json.loads(pathlib.Path(sys.argv[1]).read_text())
content = response['choices'][0]['message']['content']
assert '2 + 2 = 4' in content, content
log_path = pathlib.Path(sys.argv[2])
assert log_path.exists(), f'missing log {log_path}'
lines = [json.loads(line) for line in log_path.read_text().splitlines() if line.strip()]
statuses = {entry.get('status') for entry in lines}
assert 'retrying' in statuses, statuses
assert 'approved' in statuses, statuses
print('E2E demo OK: proxy rejected first answer, retried, approved corrected answer, and wrote logs')
PY

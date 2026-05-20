#!/usr/bin/env python3
"""Tiny OpenAI-compatible mock upstream for The Witness demos.

It intentionally returns one weak answer before returning a better answer when
The Witness retries with a repaired prompt. No external API keys are required.
"""

from __future__ import annotations

import argparse
import json
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from typing import Any


class MockOpenAIHandler(BaseHTTPRequestHandler):
    server_version = "WitnessMockOpenAI/0.1"

    def do_GET(self) -> None:  # noqa: N802 - stdlib handler API
        if self.path in {"/", "/health", "/v1/models"}:
            self._json(200, {"status": "ok", "models": [{"id": "mock"}]})
            return
        self._json(404, {"error": "not found"})

    def do_POST(self) -> None:  # noqa: N802 - stdlib handler API
        if self.path != "/v1/chat/completions":
            self._json(404, {"error": "not found"})
            return

        body = self._read_json()
        messages = body.get("messages", []) if isinstance(body, dict) else []
        joined = "\n".join(str(m.get("content", "")) for m in messages if isinstance(m, dict))
        repaired = "previous answer was rejected" in joined.lower() or "required fix" in joined.lower()
        content = (
            "2 + 2 = 4 because adding two items to two more items gives a total of four items."
            if repaired
            else "2 + 2 equals 5 because numbers are flexible."
        )
        self._json(
            200,
            {
                "id": "chatcmpl-witness-mock",
                "object": "chat.completion",
                "model": body.get("model", "mock") if isinstance(body, dict) else "mock",
                "choices": [{"index": 0, "message": {"role": "assistant", "content": content}, "finish_reason": "stop"}],
            },
        )

    def log_message(self, format: str, *args: Any) -> None:  # noqa: A002 - stdlib override name
        print(f"mock-upstream {self.address_string()} - {format % args}")

    def _read_json(self) -> dict[str, Any]:
        length = int(self.headers.get("content-length", "0") or "0")
        raw = self.rfile.read(length) if length else b"{}"
        try:
            data = json.loads(raw.decode("utf-8"))
        except json.JSONDecodeError:
            return {}
        return data if isinstance(data, dict) else {}

    def _json(self, status: int, payload: dict[str, Any]) -> None:
        data = json.dumps(payload).encode("utf-8")
        self.send_response(status)
        self.send_header("content-type", "application/json")
        self.send_header("content-length", str(len(data)))
        self.end_headers()
        self.wfile.write(data)


def main() -> None:
    parser = argparse.ArgumentParser(description="Run a local OpenAI-compatible mock upstream for The Witness demos.")
    parser.add_argument("--host", default="127.0.0.1", help="Bind host. Default: 127.0.0.1")
    parser.add_argument("--port", type=int, default=8000, help="Bind port. Default: 8000")
    args = parser.parse_args()
    server = ThreadingHTTPServer((args.host, args.port), MockOpenAIHandler)
    print(f"Mock upstream listening on http://{args.host}:{args.port}/v1")
    server.serve_forever()


if __name__ == "__main__":
    main()

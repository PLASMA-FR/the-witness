# The Witness status

| Feature | Status | Notes |
|---|---|---|
| CLI | Working | Tested on Linux. Existing commands preserved. |
| TUI | Working | Existing TUI preserved; Linux smoke/layout tests run. |
| Web UI | Working | React/Vite dashboard builds and is served by `the-witness dashboard`. |
| Dashboard API | Working | Local control API under `http://127.0.0.1:8790/api/*`; tested on Linux. |
| Proxy | Working | OpenAI-compatible non-streaming proxy tested with demo and Blackbox routing fix. |
| Replay/export | Working | Reads JSONL audit logs and exports Markdown/JSON/JSONL. |
| Linux installer | Working/tested | `scripts/install.sh` syntax and safe install flow tested on Linux. |
| Windows installer | Script created | PowerShell script created; syntax read check possible on Linux only if `pwsh` is installed. Needs Windows validation. |
| Linux systemd user service | Created | Code writes `~/.config/systemd/user/the-witness.service`; status checked best-effort on Linux. |
| macOS launchd service | Created/untested | Plist generation implemented; not tested on macOS in this environment. |
| Windows service/task | Created/untested | Scheduled Task fallback implemented; not tested on Windows in this environment. |
| Ollama backend | Supported | Default backend. Runtime depends on local Ollama and pulled models. |
| gemma4:e2b | Default | Confirmed default judge model. |
| gemma4:e4b | Strong/high-risk | Confirmed high-risk judge model. |
| llama.cpp backend | Supported | Config/test path supported; live server not tested unless user runs one. |
| LiteRT backend | Experimental/supported | Setup surface exists; runtime not fully tested here. |
| Unsloth/HF model | Supported | HF link documented: https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge |
| Blackbox endpoint | Working | Uses `BLACKBOX_API_KEY`; real upstream/proxy path previously verified. |
| Cactus | Not claimed | Not part of this release. |

Platform actually tested: Linux.

Platforms not actually tested here: Windows and macOS.

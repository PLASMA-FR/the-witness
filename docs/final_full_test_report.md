# The Witness final release audit report

The Witness is a local-first Gemma 4 reliability firewall for AI endpoints. This report records the release audit work completed in `/home/admin/Gemma/witness` and separates verified results from optional setup that still depends on the user environment.

## Summary

The project was checked from the point of view of a user, hackathon judge, Rust maintainer, TUI operator, Web UI reviewer, service operator, and security reviewer.

Verified in this pass:

- Rust formatting, tests, clippy, release build, and CLI smoke checks.
- Web UI TypeScript build/lint and browser verification.
- Dashboard API health checks.
- Desktop dashboard, endpoints, requests, detail, repair, review, models, logs, and doctor screens.
- Mobile dashboard rendering at 390px width.
- Service install/status/uninstall workflow.
- Installer safe mode.
- Demo E2E script.
- Dataset and notebook structural validation.
- Secret redaction behavior for documented Blackbox usage.
- Port-conflict warnings through dashboard startup and doctor checks.

Fixed during the audit:

- Removed generated `web/dist` build files from Git tracking and added `web/dist/` to `.gitignore`.
- Added missing control API aliases for `/api/settings` and `/api/system/status`.
- Improved dashboard spacing where dense rows were colliding.
- Added a visible `Edit` action to endpoint cards.
- Normalized Blackbox examples to environment-variable placeholders instead of token-looking values.
- Added a mock OpenAI-compatible upstream server for repeatable local demos.
- Improved missing-audit-log behavior for replay/export commands.
- Added refreshed Web UI screenshots.

## Commands and results

| Area | Command or check | Result |
|---|---|---|
| Rust formatting | `cargo fmt --check` | Passed |
| Rust lint | `cargo clippy --all-targets --all-features -- -D warnings` | Passed |
| Rust tests | `cargo test --all -- --nocapture` | Passed |
| Release build | `cargo build --release` | Passed |
| Rust check | `cargo check --all-targets --all-features` | Passed |
| Web build | `npm run build` in `web/` | Passed; Vite reported a large-chunk warning only |
| Web lint/typecheck | `npm run lint` in `web/` | Passed |
| Dataset validation | `python3 training/scripts/validate_dataset.py` | Passed |
| Notebook structure | JSON validation for E2B/E4B notebooks | Passed |
| Python syntax | `python3 -m py_compile` on training/demo scripts | Passed |
| Shell syntax | `bash -n` on shell scripts | Passed |
| Installer safe mode | `WITNESS_SKIP_OLLAMA_PULL=1 bash scripts/install.sh` | Passed |
| Verify script | `bash scripts/verify.sh` | Passed after formatting was applied |
| Demo script | `bash scripts/e2e_demo.sh` | Passed; proxy rejected, retried, approved, and logged |
| Dashboard service | `./target/release/the-witness dashboard --no-open` | Started on `127.0.0.1:8790` |
| Dashboard APIs | `/api/health`, `/api/system/status`, `/api/settings`, `/api/models`, `/api/endpoints`, `/api/requests`, `/api/logs`, `/api/system/doctor` | Responded successfully on active service |
| Browser console | Browser console check | No console messages or JavaScript errors observed |
| Service install | `the-witness service install/status/uninstall` | Passed |
| Dashboard port conflict | Port 8790 occupied, then dashboard started | Reported address-in-use error |
| Proxy port warning | Port 8787 occupied, then `the-witness doctor` | Warned that local proxy port was in use with a clear fix |

## Web UI verification

Browser verification was run against the active local dashboard service at:

```text
http://127.0.0.1:8790
```

Verified pages:

- Dashboard
- Endpoints
- Requests
- Request Detail
- Prompt Repair
- Human Review
- Models
- Logs
- Doctor

Screenshots saved under:

```text
web_screenshots/
```

Important refreshed screenshots:

- `web_screenshots/dashboard_desktop.png`
- `web_screenshots/dashboard_mobile.png`
- `web_screenshots/endpoints_desktop.png`

Desktop result:

- Dashboard is readable, useful, and visually coherent.
- Charts render correctly.
- Demo mode is clearly labeled.
- No raw secrets were visible.
- Endpoint cards now expose Edit, Test, Copy curl, View requests, and Delete.
- Doctor shows fix commands for common setup issues.

Mobile result:

- Dashboard was captured at 390px width with Chromium headless.
- The page is readable and usable.
- The bottom navigation is visible.
- No raw secrets were visible.
- `models_mobile.png` was not counted as verified Models mobile evidence because the app is state-driven and does not route by query string.

## CLI and service verification

Verified CLI areas:

- Top-level help.
- Model help and demo judge model test.
- Endpoint list/help.
- Doctor health checks.
- Replay/export missing-log behavior.
- Dashboard service startup.
- User service install/status/uninstall.

The TUI requires a real TTY. Non-interactive `the-witness start` correctly explains that users should run `the-witness doctor` for automated health checks.

## Security and privacy review

Security checks focused on practical release risks:

- No real API keys should be committed.
- Blackbox examples use `BLACKBOX_API_KEY` as an environment variable.
- The Web UI shows environment variable names, not secret values.
- `.env`, model weights, generated Web builds, and common credential files are ignored or treated as non-release artifacts.
- Doctor output gives safe setup instructions without printing secrets.
- Request logs are designed around redaction and JSONL audit trails.

One important operational rule remains: users should export secrets in their shell or service environment and should not paste real keys into config files.

## Optional/user-side setup

The following are intentionally user-side or environment-dependent:

- Pulling `gemma4:e4b` for stronger/high-risk judging.
- Setting `BLACKBOX_API_KEY` before using the Blackbox endpoint.
- Installing Hugging Face CLI for model download convenience.
- Downloading the fine-tuned Witness LoRA adapter when selected.
- Running llama.cpp, LiteRT, or Unsloth runtime tests in the user's target environment.

The audit verified project code paths and documented setup, but it did not claim runtime success for external runtimes that were not launched in this environment.

## Hackathon assets and links

Required links are present in the README/docs:

- Fine-tuned model: `https://huggingface.co/ahmadalfakeh/witness-gemma4-e2b-judge`
- Colab notebook: `https://colab.research.google.com/drive/17-CgEQLNg8bpnhhWzJwpapRxQyHIqybq?usp=sharing`

The project is positioned as:

```text
A local-first Gemma 4 reliability firewall for AI endpoints.
```

Primary impact track:

```text
Safety & Trust
```

Technology tracks represented in setup/docs:

- Ollama
- llama.cpp
- LiteRT
- Unsloth

## Known notes

- Vite reports a large JavaScript chunk warning during Web UI build. The build passes; future work can split charts and page modules.
- PowerShell installer runtime execution was not completed because `pwsh` was not available in this environment.
- The dashboard port-conflict path reports the bind error clearly, but the log prints startup context before the final address-in-use error.
- The fine-tuned E2B adapter is published as an adapter path; users still need the compatible Gemma base model/runtime.

## Release judgment

The audited local release is in strong shape for a hackathon submission. The tested paths build, run, and present the core story clearly: The Witness watches AI endpoints, judges responses with Gemma, blocks risky answers, repairs prompts, retries safely, and leaves an audit trail.

Final release completion still requires the final Git commit and push attempt after this report and final scans are complete.

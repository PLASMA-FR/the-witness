# The Witness on macOS

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
```

Install Rust first if `cargo` is missing: https://rustup.rs/

## Run

```bash
the-witness dashboard
# open http://127.0.0.1:8790

the-witness start
```

## Service

macOS uses a launchd user agent:

```bash
the-witness service install
the-witness service start
the-witness service status
```

Plist:

```text
~/Library/LaunchAgents/com.thewitness.dashboard.plist
```

This was created from Linux and must be verified on macOS before production claims.

## Models

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

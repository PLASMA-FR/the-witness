# The Witness on Linux

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.sh | bash
```

Or from a checkout:

```bash
cargo build --release
cargo install --path .
```

## Run

```bash
the-witness dashboard
# open http://127.0.0.1:8790

the-witness start
```

## Service

Linux uses a systemd user service:

```bash
the-witness service install
the-witness service start
the-witness service status
journalctl --user -u the-witness -f
```

## Models

```bash
ollama pull gemma4:e2b
ollama pull gemma4:e4b
```

## Blackbox

```bash
export BLACKBOX_API_KEY="YOUR_KEY_HERE"
the-witness endpoint add-blackbox
```

# The Witness on Windows

## Quick install

```powershell
powershell -ExecutionPolicy Bypass -Command "irm https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.ps1 | iex"
```

Safer inspect-first install:

```powershell
curl.exe -L https://raw.githubusercontent.com/PLASMA-FR/the-witness/main/scripts/install.ps1 -o install.ps1
notepad install.ps1
powershell -ExecutionPolicy Bypass -File .\install.ps1
```

## Run

```powershell
the-witness.exe dashboard
# open http://127.0.0.1:8790

the-witness.exe start
```

## Service / scheduled task

The Windows implementation uses a per-user Scheduled Task fallback named `TheWitness`.

```powershell
the-witness.exe service install
the-witness.exe service start
the-witness.exe service status
the-witness.exe service stop
the-witness.exe service uninstall
```

This was authored from Linux and needs validation on Windows before production claims.

## Blackbox endpoint (Example API)

```powershell
$env:BLACKBOX_API_KEY="YOUR_KEY_HERE"
the-witness.exe endpoint add-blackbox
```

Secrets must remain environment variables or local private config only.

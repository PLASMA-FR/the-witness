# The Witness background service

The service runs:

```bash
the-witness dashboard --no-open
```

The dashboard/control API binds to `127.0.0.1:8790` by default and the proxy remains local by default.

## Linux: systemd user service

```bash
the-witness service install
the-witness service start
the-witness service status
the-witness service logs
```

Unit path:

```text
~/.config/systemd/user/the-witness.service
```

View logs:

```bash
journalctl --user -u the-witness -f
```

Stop/remove:

```bash
the-witness service stop
the-witness service uninstall
```

## macOS: launchd user agent

```bash
the-witness service install
the-witness service start
the-witness service status
```

Plist path:

```text
~/Library/LaunchAgents/com.thewitness.dashboard.plist
```

Stop/remove:

```bash
the-witness service stop
the-witness service uninstall
```

Status/log checks are best-effort from Linux development; verify on macOS before production rollout.

## Windows: scheduled task fallback

The Windows implementation uses a per-user Scheduled Task named `TheWitness` so admin rights are not required.

```powershell
the-witness.exe service install
the-witness.exe service start
the-witness.exe service status
the-witness.exe service logs
```

Remove:

```powershell
the-witness.exe service stop
the-witness.exe service uninstall
```

Native Windows service wrappers such as NSSM or WinSW can also run:

```powershell
the-witness.exe dashboard --no-open
```

## Security

- Dashboard/control API is localhost-only by default.
- Do not bind to `0.0.0.0` unless you fully trust the network.
- API responses redact stored secret values.
- Prefer `bearer_env` or `header_env` endpoint auth.

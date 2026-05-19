param(
    [string]$RepoUrl = "https://github.com/PLASMA-FR/the-witness.git",
    [string]$InstallDir = "$env:USERPROFILE\.the-witness\bin",
    [string]$ConfigDir = "$env:APPDATA\TheWitness",
    [switch]$InstallService
)

$ErrorActionPreference = "Stop"

function Info($m) { Write-Host "[info] $m" -ForegroundColor Cyan }
function Warn($m) { Write-Host "[warn] $m" -ForegroundColor Yellow }
function Have($cmd) { return [bool](Get-Command $cmd -ErrorAction SilentlyContinue) }

Info "The Witness Windows installer"
Info "No API keys are required. Do not paste secrets into this installer."

if (-not (Have git)) {
    throw "git is required. Install Git for Windows, then rerun this script."
}
if (-not (Have cargo)) {
    Write-Host "Rust/Cargo is missing. Install Rust from https://rustup.rs/ then rerun this installer." -ForegroundColor Red
    exit 1
}

$Current = Get-Location
if ((Test-Path (Join-Path $Current "Cargo.toml")) -and (Select-String -Path (Join-Path $Current "Cargo.toml") -Pattern 'name = "the-witness"' -Quiet)) {
    $WorkDir = $Current.Path
    Info "Using current checkout: $WorkDir"
} else {
    $WorkDir = Join-Path $env:TEMP "the-witness-install"
    if (Test-Path $WorkDir) { Remove-Item -Recurse -Force $WorkDir }
    Info "Cloning $RepoUrl"
    git clone --depth 1 $RepoUrl $WorkDir
}

Push-Location $WorkDir
try {
    Info "Building release binary"
    cargo build --release
    New-Item -ItemType Directory -Force -Path $InstallDir | Out-Null
    New-Item -ItemType Directory -Force -Path $ConfigDir | Out-Null
    Copy-Item -Force "target\release\the-witness.exe" (Join-Path $InstallDir "the-witness.exe")
    if ((Test-Path "witness.toml") -and -not (Test-Path (Join-Path $ConfigDir "witness.toml"))) {
        Copy-Item "witness.toml" (Join-Path $ConfigDir "witness.toml")
    }
} finally {
    Pop-Location
}

Info "Installed: $(Join-Path $InstallDir 'the-witness.exe')"
Warn "Add this directory to PATH if needed: $InstallDir"
Warn "Pull local judge models when Ollama is installed:"
Write-Host "  ollama pull gemma4:e2b"
Write-Host "  ollama pull gemma4:e4b"

if ($InstallService) {
    Info "Creating user scheduled task TheWitness"
    $Exe = Join-Path $InstallDir "the-witness.exe"
    schtasks /Create /TN "TheWitness" /SC ONLOGON /TR "`"$Exe`" dashboard --no-open" /F | Out-Null
}

Write-Host ""
Write-Host "The Witness install complete."
Write-Host "Run Web Dashboard: the-witness.exe dashboard"
Write-Host "Open: http://127.0.0.1:8790"
Write-Host "Run TUI: the-witness.exe start"
Write-Host "Service commands: the-witness.exe service install/start/status"
Write-Host "Blackbox: `$env:BLACKBOX_API_KEY='YOUR_KEY_HERE'; the-witness.exe endpoint add-blackbox"

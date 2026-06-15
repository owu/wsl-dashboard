#!/usr/bin/env pwsh

Write-Host "Building Windows executable for WSL Dashboard..." -ForegroundColor Green

# Check rustup toolchain
$defaultToolchain = rustup default 2>&1
if ($defaultToolchain -notmatch "x86_64-pc-windows-msvc") {
    Write-Host "[!] Error: Current default toolchain is not x86_64-pc-windows-msvc" -ForegroundColor Red
    Write-Host "To ensure packaging compatibility and reduce dependencies, this project requires building with the x86_64-pc-windows-msvc toolchain." -ForegroundColor Yellow
    Write-Host "`nPlease follow these steps to install and set as default:" -ForegroundColor White
    Write-Host "1. Install toolchain: " -NoNewline; Write-Host "rustup toolchain install stable-x86_64-pc-windows-msvc" -ForegroundColor Cyan
    Write-Host "2. Set as default: " -NoNewline; Write-Host "rustup default stable-x86_64-pc-windows-msvc" -ForegroundColor Cyan
    Write-Host "`nRe-run this script after installation is complete." -ForegroundColor Yellow
    exit 1
}

# Read version from Cargo.toml
$cargoToml = Get-Content -Path "$PSScriptRoot/../../Cargo.toml" -Raw
$versionMatch = [regex]::Match($cargoToml, 'version\s*=\s*"([^"]+)"')
if (-not $versionMatch.Success) {
    Write-Host "Could not find version in Cargo.toml" -ForegroundColor Red
    exit 1
}
$version = $versionMatch.Groups[1].Value
Write-Host "Detected version: $version" -ForegroundColor Cyan

# Enter root directory and build
Push-Location "$PSScriptRoot/../.."

# Clean cache
# cargo clean

Write-Host "Compiling..." -ForegroundColor Yellow
cargo build --release

# Memory leak check
# cargo build --release --features dhat-heap

if ($LASTEXITCODE -ne 0) {
    Write-Host "Build failed!" -ForegroundColor Red
    Pop-Location
    exit 1
}

# Prepare release directory
$releaseDir = "./build/releases"
if (-not (Test-Path $releaseDir)) {
    New-Item -ItemType Directory -Path $releaseDir -Force
}

# Copy and rename executable
# Use default release directory (determined by system default toolchain x86_64-pc-windows-msvc)
$sourcePath = "./target/release/wsldashboard.exe"
$destinationPath = "$releaseDir/WSLDashboard.$version.Portable.x64.exe"
$zipPath = "$releaseDir/WSLDashboard.$version.Portable.x64.zip"

Write-Host "Packaging..." -ForegroundColor Yellow
# Copy .exe
Copy-Item -Path $sourcePath -Destination $destinationPath -Force

# Create .zip
if (Test-Path $zipPath) {
    Remove-Item $zipPath -Force
}
Compress-Archive -Path $sourcePath -DestinationPath $zipPath -CompressionLevel Optimal

Pop-Location

Write-Host "--- Build completed! ---" -ForegroundColor Green
Write-Host "Executable: $destinationPath" -ForegroundColor Cyan
Write-Host "Archive:    $zipPath" -ForegroundColor Cyan

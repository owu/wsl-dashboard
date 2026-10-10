#!/usr/bin/env pwsh

param(
    [Parameter(Mandatory = $false)]
    [ValidateSet("x64", "arm64")]
    [string]$Arch = "x64"
)

Write-Host "Building Windows executable for WSL Dashboard ($Arch)..." -ForegroundColor Green

# Check rustup toolchain
# The host toolchain must stay x86_64-pc-windows-msvc: x64 is built natively into target/release,
# while arm64 is cross-compiled from this host via --target.
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

# Resolve target triple and artifact suffix from -Arch
$Target = $null
$Suffix = "x64"
if ($Arch -eq "arm64") {
    $Target = "aarch64-pc-windows-msvc"
    $Suffix = "arm64"

    # ring 0.17 always drives `clang` when compiling its C code for
    # aarch64-pc-windows-msvc (see ring/build.rs, "On Windows AArch64 we
    # currently must use Clang"). A clang that cannot locate the MSVC / Windows
    # SDK headers dies with "'assert.h' file not found", so expose them here.
    if (-not $env:INCLUDE) {
        $vswhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
        $vsPath = & $vswhere -latest -products * -requires Microsoft.VisualStudio.Component.VC.Tools.ARM64 -property installationPath 2>$null
        if (-not $vsPath) {
            $vsPath = & $vswhere -latest -products * -property installationPath 2>$null
        }

        $msvcInclude = $null
        $msvcRoot = Join-Path $vsPath "VC\Tools\MSVC"
        if ($vsPath -and (Test-Path $msvcRoot)) {
            $msvcInclude = Get-ChildItem $msvcRoot -Directory | Sort-Object Name -Descending |
                ForEach-Object { Join-Path $_.FullName "include" } |
                Where-Object { Test-Path $_ } | Select-Object -First 1
        }

        $kitsRoot = (Get-ItemProperty "HKLM:\SOFTWARE\WOW6432Node\Microsoft\Windows Kits\Installed Roots" -ErrorAction SilentlyContinue).KitsRoot10
        $sdkInclude = $null
        if ($kitsRoot) {
            $sdkInclude = Get-ChildItem (Join-Path $kitsRoot "Include") -Directory -ErrorAction SilentlyContinue | Sort-Object Name -Descending |
                ForEach-Object { $_.FullName } |
                Where-Object { Test-Path (Join-Path $_ "ucrt\assert.h") } | Select-Object -First 1
        }

        if ($msvcInclude -and $sdkInclude) {
            $env:INCLUDE = "$msvcInclude;$sdkInclude\ucrt;$sdkInclude\shared;$sdkInclude\um"
            Write-Host "clang header search path: $env:INCLUDE" -ForegroundColor DarkGray
        } else {
            Write-Host "[!] Could not locate the MSVC ARM64 / Windows SDK headers." -ForegroundColor Yellow
            Write-Host "    If the build fails with 'assert.h' file not found, run this from a Developer Command Prompt." -ForegroundColor Yellow
        }
    }
}

Write-Host "Compiling..." -ForegroundColor Yellow
if ($Target) {
    cargo build --release --target $Target
} else {
    cargo build --release
}

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

# Locate the built executable
# x64: default toolchain output (target/release)
# arm64: cross-compiled output (target/aarch64-pc-windows-msvc/release)
if ($Target) {
    $sourcePath = "./target/$Target/release/wsldashboard.exe"
} else {
    $sourcePath = "./target/release/wsldashboard.exe"
}
$destinationPath = "$releaseDir/WSLDashboard.$version.Portable.$Suffix.exe"
$zipPath = "$releaseDir/WSLDashboard.$version.Portable.$Suffix.zip"

Write-Host "Packaging..." -ForegroundColor Yellow
# Copy .exe
Copy-Item -Path $sourcePath -Destination $destinationPath -Force

# Create .zip
# Archive the renamed copy, not the raw wsldashboard.exe, so the entry inside
# the zip carries the versioned name (WSLDashboard.<ver>.Portable.<arch>.exe).
if (Test-Path $zipPath) {
    Remove-Item $zipPath -Force
}
Compress-Archive -Path $destinationPath -DestinationPath $zipPath -CompressionLevel Optimal

Pop-Location

Write-Host "--- Build completed! ---" -ForegroundColor Green
Write-Host "Executable: $destinationPath" -ForegroundColor Cyan
Write-Host "Archive:    $zipPath" -ForegroundColor Cyan

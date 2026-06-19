param(
    [Parameter(Mandatory = $false)]
    [string]$Version,

    [Parameter(Mandatory = $false)]
    [string]$OutputDir,

    # Submit to winget-pkgs via wingetcreate (requires WINGET_GITHUB_TOKEN env var)
    [Parameter(Mandatory = $false)]
    [switch]$Submit,

    # GitHub PAT with public_repo scope (overrides env var)
    [Parameter(Mandatory = $false)]
    [string]$Token
)

# WSL Dashboard Winget Manifest Generator
# Generates manifest files for winget-pkgs submission

$PACKAGE_ID = "Owu.WSLDashboard"
$PUBLISHER = "owu"
$PACKAGE_NAME = "WSL Dashboard"
$AUTHOR = "owu"
$LICENSE = "GPL-3.0"
$LICENSE_URL = "https://github.com/owu/wsl-dashboard/blob/main/LICENSE"
$COPYRIGHT = "Copyright 2026 WSL Dashboard"
$HOMEPAGE = "https://www.wslui.com"
$REPO = "https://github.com/owu/wsl-dashboard"
$SHORT_DESCRIPTION = "A modern, high-performance, lightweight, and low-memory WSL instance management dashboard."
$TAGS = @("wsl", "windows-subsystem-for-linux", "linux", "dashboard", "gui", "management", "rust")
$INSTALLER_TYPE = "inno"
$ARCH = "x64"
$SCOPE = "machine"
$MIN_WINDOWS_VERSION = "10.0.17763.0"  # Windows 10 1809+

# Read version from Cargo.toml if not provided
if ([string]::IsNullOrWhiteSpace($Version)) {
    $cargoToml = Get-Content -Path "$PSScriptRoot/../../Cargo.toml" -Raw
    $versionMatch = [regex]::Match($cargoToml, 'version\s*=\s*"([^"]+)"')
    if (-not $versionMatch.Success) {
        Write-Host "Could not find version in Cargo.toml" -ForegroundColor Red
        exit 1
    }
    $VERSION = $versionMatch.Groups[1].Value
}

Write-Host "--- Generating Winget Manifest for $PACKAGE_NAME v$VERSION ---" -ForegroundColor Cyan

# ---- Submit mode: use wingetcreate to update and submit PR ----
if ($Submit) {
    # Resolve token
    $ghToken = if ($Token) { $Token } elseif ($env:WINGET_GITHUB_TOKEN) { $env:WINGET_GITHUB_TOKEN } else { $null }
    if ([string]::IsNullOrWhiteSpace($ghToken)) {
        Write-Error "No GitHub token provided. Set WINGET_GITHUB_TOKEN env var or use -Token parameter."
        Write-Host "Token needs 'public_repo' scope to create PRs in microsoft/winget-pkgs." -ForegroundColor Yellow
        exit 1
    }

    # Check for wingetcreate
    $wingetcreate = Get-Command wingetcreate -ErrorAction SilentlyContinue
    if (-not $wingetcreate) {
        Write-Host "wingetcreate not found, downloading..." -ForegroundColor Yellow
        $downloadUrl = "https://aka.ms/wingetcreate/latest"
        $wingetcreatePath = Join-Path $env:TEMP "wingetcreate.exe"
        Invoke-WebRequest -Uri $downloadUrl -OutFile $wingetcreatePath
        $wingetcreate = $wingetcreatePath
    }
    else {
        $wingetcreate = $wingetcreate.Source
    }

    $installerUrl = "$REPO/releases/download/v$VERSION/WSLDashboard.$VERSION.Setup.x64.exe"
    Write-Host "Installer URL: $installerUrl" -ForegroundColor Gray
    Write-Host "Submitting to winget-pkgs..." -ForegroundColor Yellow

    & $wingetcreate update $PACKAGE_ID `
        --version $VERSION `
        --urls $installerUrl `
        --submit `
        --token $ghToken

    if ($LASTEXITCODE -ne 0) {
        Write-Error "wingetcreate submission failed!"
        exit 1
    }

    Write-Host "--- Successfully submitted $PACKAGE_NAME v$VERSION to winget-pkgs! ---" -ForegroundColor Green
    Write-Host "A PR has been created at: https://github.com/microsoft/winget-pkgs/pulls" -ForegroundColor Cyan
    exit 0
}

# ---- Generate mode: create local manifest files ----

# 1. Locate the Setup installer
$SETUP_EXE = Join-Path $PSScriptRoot "..\..\build\releases\WSLDashboard.$VERSION.Setup.x64.exe"
if (-not (Test-Path $SETUP_EXE)) {
    Write-Error "Setup installer not found at $SETUP_EXE"
    Write-Host "Please run '.\build\setup\build.ps1' first to build the installer." -ForegroundColor Yellow
    exit 1
}
Write-Host "Step 1: Found installer at $SETUP_EXE" -ForegroundColor Gray

# 2. Calculate SHA256
Write-Host "Step 2: Calculating SHA256 hash..." -ForegroundColor Yellow
$hash = (Get-FileHash -Path $SETUP_EXE -Algorithm SHA256).Hash
Write-Host "SHA256: $hash" -ForegroundColor Gray

# 3. Prepare output directory
if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = Join-Path $PSScriptRoot "..\..\build\winget\$PACKAGE_ID"
}
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}
Write-Host "Step 3: Output directory: $OutputDir" -ForegroundColor Gray

# 4. Construct installer URL (GitHub release)
$INSTALLER_URL = "$REPO/releases/download/v$VERSION/WSLDashboard.$VERSION.Setup.x64.exe"

# 5. Generate version manifest
$versionManifest = @"
# yaml-language-server: `$schema=https://aka.ms/winget-manifest.version.1.9.0.schema.json
PackageIdentifier: $PACKAGE_ID
PackageVersion: $VERSION
DefaultLocale: en-US
ManifestType: version
ManifestVersion: 1.9.0
"@

$versionFile = Join-Path $OutputDir "$PACKAGE_ID.yaml"
Set-Content -Path $versionFile -Value $versionManifest -Encoding UTF8
Write-Host "Generated: $versionFile" -ForegroundColor Green

# 6. Generate installer manifest
$installerManifest = @"
# yaml-language-server: `$schema=https://aka.ms/winget-manifest.installer.1.9.0.schema.json
PackageIdentifier: $PACKAGE_ID
PackageVersion: $VERSION
Platform:
- Windows.Desktop
MinimumOSVersion: $MIN_WINDOWS_VERSION
InstallerType: $INSTALLER_TYPE
Scope: $SCOPE
InstallModes:
- interactive
- silent
- silentWithProgress
InstallerSwitches:
  Silent: /VERYSILENT /LANG=english
  SilentWithProgress: /SILENT /LANG=english
  Custom: /NORESTART
UpgradeBehavior: install
ElevationRequirement: elevationRequired
ProductCode: "{5CCAB770-FE6B-4A69-9486-74C5D24D3860}_is1"
Installers:
- Architecture: $ARCH
  InstallerUrl: $INSTALLER_URL
  InstallerSha256: $hash
ManifestType: installer
ManifestVersion: 1.9.0
"@

$installerFile = Join-Path $OutputDir "$PACKAGE_ID.installer.yaml"
Set-Content -Path $installerFile -Value $installerManifest -Encoding UTF8
Write-Host "Generated: $installerFile" -ForegroundColor Green

# 7. Generate en-US locale manifest
$localeEn = @"
# yaml-language-server: `$schema=https://aka.ms/winget-manifest.defaultLocale.1.9.0.schema.json
PackageIdentifier: $PACKAGE_ID
PackageVersion: $VERSION
PackageLocale: en-US
Publisher: $PUBLISHER
PublisherUrl: $REPO
PublisherSupportUrl: $REPO/issues
Author: $AUTHOR
PackageName: $PACKAGE_NAME
PackageUrl: $HOMEPAGE
License: $LICENSE
LicenseUrl: $LICENSE_URL
Copyright: $COPYRIGHT
ShortDescription: $SHORT_DESCRIPTION
Description: >-
  WSL Dashboard is a modern, high-performance, lightweight, and low-memory
  WSL (Windows Subsystem for Linux) instance management dashboard built with
  Rust and Slint UI. It provides a native Windows GUI for managing your WSL
  distributions, including creating, deleting, cloning, exporting, importing,
  compressing, and moving WSL instances. Features also include network port
  forwarding management, HTTP proxy configuration, USB device passthrough,
  and multi-language support with 50+ languages.
Tags:
- $($TAGS -join "`n- ")
ReleaseNotesUrl: $REPO/releases/tag/v$VERSION
ManifestType: defaultLocale
ManifestVersion: 1.9.0
"@

$localeFile = Join-Path $OutputDir "$PACKAGE_ID.locale.en-US.yaml"
Set-Content -Path $localeFile -Value $localeEn -Encoding UTF8
Write-Host "Generated: $localeFile" -ForegroundColor Green

# 8. Generate zh-CN locale manifest (optional, primary user base)
$localeZhCn = @"
# yaml-language-server: `$schema=https://aka.ms/winget-manifest.locale.1.9.0.schema.json
PackageIdentifier: $PACKAGE_ID
PackageVersion: $VERSION
PackageLocale: zh-CN
Publisher: $PUBLISHER
PublisherUrl: $REPO
PublisherSupportUrl: $REPO/issues
Author: $AUTHOR
PackageName: $PACKAGE_NAME
PackageUrl: $HOMEPAGE
License: $LICENSE
LicenseUrl: $LICENSE_URL
Copyright: $COPYRIGHT
ShortDescription: "一款现代、高性能、轻量级且低内存占用的 WSL 实例管理仪表板。基于 Rust 和 Slint 构建，提供顶级的原生体验。"
Description: >-
  WSL Dashboard 是一款基于 Rust 和 Slint UI 构建的现代化 WSL 实例管理面板。
  提供原生 Windows 图形界面，支持创建、删除、克隆、导出、导入、压缩和迁移
  WSL 发行版。还支持网络端口转发管理、HTTP 代理配置、USB 设备直通，
  以及 50+ 语言的多语言支持。
Tags:
- wsl
- windows-subsystem-for-linux
- linux
- dashboard
- gui
- management
ReleaseNotesUrl: $REPO/releases/tag/v$VERSION
ManifestType: locale
ManifestVersion: 1.9.0
"@

$localeZhCnFile = Join-Path $OutputDir "$PACKAGE_ID.locale.zh-CN.yaml"
Set-Content -Path $localeZhCnFile -Value $localeZhCn -Encoding UTF8
Write-Host "Generated: $localeZhCnFile" -ForegroundColor Green

Write-Host "`n--- Winget manifest generation complete! ---" -ForegroundColor Green
Write-Host "Files generated in: $OutputDir" -ForegroundColor Cyan
Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "  1. Validate: winget validate --manifest $OutputDir" -ForegroundColor White
Write-Host "  2. Test install: winget install --manifest $OutputDir" -ForegroundColor White
Write-Host "  3. Submit to https://github.com/microsoft/winget-pkgs" -ForegroundColor White

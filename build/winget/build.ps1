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
$PUBLISHER = "https://github.com/owu"
$PACKAGE_NAME = "WSL Dashboard"
$AUTHOR = "owu"
$LICENSE = "GPL-3.0"
$LICENSE_URL = "https://github.com/owu/wsl-dashboard/blob/main/LICENSE"
$COPYRIGHT = "2026 WSL Dashboard. All rights reserved."
$HOMEPAGE = "https://www.wslui.com"
$PRIVACY_URL = "https://www.wslui.com/privacy/"
$REPO = "https://github.com/owu/wsl-dashboard"
$SHORT_DESCRIPTION = "A modern, high-performance, lightweight, and low-memory WSL instance management dashboard."
$TAGS = @("wsl", "windows-subsystem-for-linux", "linux", "dashboard", "gui", "management", "rust")
$INSTALLER_TYPE = "inno"
$SCOPE = "machine"

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

# 1. Locate the Setup installers (x64 required, arm64 optional)
$X64_EXE = Join-Path $PSScriptRoot "..\..\build\releases\WSLDashboard.$VERSION.Setup.x64.exe"
if (-not (Test-Path $X64_EXE)) {
    Write-Error "Setup installer not found at $X64_EXE"
    Write-Host "Please run '.\build\setup\build.ps1' first to build the installer." -ForegroundColor Yellow
    exit 1
}
Write-Host "Step 1: Found x64 installer at $X64_EXE" -ForegroundColor Gray

$ARM64_EXE = Join-Path $PSScriptRoot "..\..\build\releases\WSLDashboard.$VERSION.Setup.arm64.exe"
$hasArm64 = Test-Path $ARM64_EXE
if ($hasArm64) {
    Write-Host "Step 1: Found arm64 installer at $ARM64_EXE" -ForegroundColor Gray
} else {
    Write-Host "Step 1: arm64 installer not found, generating x64-only manifest" -ForegroundColor Yellow
}

# 2. Calculate SHA256
Write-Host "Step 2: Calculating SHA256 hash..." -ForegroundColor Yellow
$hashX64 = (Get-FileHash -Path $X64_EXE -Algorithm SHA256).Hash
Write-Host "SHA256 (x64): $hashX64" -ForegroundColor Gray
if ($hasArm64) {
    $hashArm64 = (Get-FileHash -Path $ARM64_EXE -Algorithm SHA256).Hash
    Write-Host "SHA256 (arm64): $hashArm64" -ForegroundColor Gray
}

# 3. Prepare output directory
if ([string]::IsNullOrWhiteSpace($OutputDir)) {
    $OutputDir = Join-Path $PSScriptRoot "..\..\build\winget\$PACKAGE_ID"
}
if (-not (Test-Path $OutputDir)) {
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
}
Write-Host "Step 3: Output directory: $OutputDir" -ForegroundColor Gray

# 5. Generate version manifest
$versionManifest = @"
# Created using wingetcreate 1.12.8.0
# yaml-language-server: `$schema=https://aka.ms/winget-manifest.version.1.12.0.schema.json

PackageIdentifier: $PACKAGE_ID
PackageVersion: $VERSION
DefaultLocale: en-US
ManifestType: version
ManifestVersion: 1.12.0
"@

$versionFile = Join-Path $OutputDir "$PACKAGE_ID.yaml"
Set-Content -Path $versionFile -Value $versionManifest -Encoding UTF8
Write-Host "Generated: $versionFile" -ForegroundColor Green

# 6. Generate installer manifest (one installer entry per architecture)
$installerEntries = @(
    @"
- Architecture: x64
  InstallerUrl: $REPO/releases/download/v$VERSION/WSLDashboard.$VERSION.Setup.x64.exe
  InstallerSha256: $hashX64
  Dependencies:
    PackageDependencies:
      - PackageIdentifier: Microsoft.VCRedist.2015+.x64
        MinimumVersion: 14.40.33810
"@
)
if ($hasArm64) {
    $installerEntries += @"
- Architecture: arm64
  InstallerUrl: $REPO/releases/download/v$VERSION/WSLDashboard.$VERSION.Setup.arm64.exe
  InstallerSha256: $hashArm64
  Dependencies:
    PackageDependencies:
      - PackageIdentifier: Microsoft.VCRedist.2015+.arm64
        MinimumVersion: 14.40.33810
"@
}
$installersYaml = "Installers:`n" + ($installerEntries -join "`n")

$installerManifest = @"
# Created using wingetcreate 1.12.8.0
# yaml-language-server: `$schema=https://aka.ms/winget-manifest.installer.1.12.0.schema.json

PackageIdentifier: $PACKAGE_ID
PackageVersion: $VERSION
InstallerType: $INSTALLER_TYPE
Scope: $SCOPE
InstallerSwitches:
  Silent: /VERYSILENT /LANG=english /NORESTART /SUPPRESSMSGBOXES
  SilentWithProgress: /SILENT /LANG=english /NORESTART /SUPPRESSMSGBOXES
ElevationRequirement: elevationRequired
$installersYaml
ManifestType: installer
ManifestVersion: 1.12.0
ReleaseDate: $(Get-Date -Format 'yyyy-MM-dd')
"@

$installerFile = Join-Path $OutputDir "$PACKAGE_ID.installer.yaml"
Set-Content -Path $installerFile -Value $installerManifest -Encoding UTF8
Write-Host "Generated: $installerFile" -ForegroundColor Green

# 7. Generate en-US locale manifest
$localeEn = @"
# Created using wingetcreate 1.12.8.0
# yaml-language-server: `$schema=https://aka.ms/winget-manifest.defaultLocale.1.12.0.schema.json

PackageIdentifier: $PACKAGE_ID
PackageVersion: $VERSION
PackageLocale: en-US
Publisher: $PUBLISHER
PublisherUrl: $REPO
PublisherSupportUrl: $REPO/issues
PrivacyUrl: $PRIVACY_URL
Author: $AUTHOR
PackageName: $PACKAGE_NAME
PackageUrl: $HOMEPAGE
License: $LICENSE
LicenseUrl: $LICENSE_URL
Copyright: $COPYRIGHT
ShortDescription: $SHORT_DESCRIPTION
Description: WSL Dashboard is a modern, high-performance, lightweight, and low-memory WSL instance management dashboard built with Rust and Slint UI.
Moniker: wsl-dashboard
Tags:
- $($TAGS -join "`n- ")
ReleaseNotesUrl: $REPO/releases/tag/v$VERSION
ManifestType: defaultLocale
ManifestVersion: 1.12.0
"@

$localeFile = Join-Path $OutputDir "$PACKAGE_ID.locale.en-US.yaml"
Set-Content -Path $localeFile -Value $localeEn -Encoding UTF8
Write-Host "Generated: $localeFile" -ForegroundColor Green

Write-Host "`n--- Winget manifest generation complete! ---" -ForegroundColor Green
Write-Host "Files generated in: $OutputDir" -ForegroundColor Cyan
Write-Host "`nNext steps:" -ForegroundColor Yellow
Write-Host "  1. Validate: winget validate --manifest $OutputDir" -ForegroundColor White
Write-Host "  2. Test install: winget install --manifest $OutputDir" -ForegroundColor White
Write-Host "  3. Submit to https://github.com/microsoft/winget-pkgs" -ForegroundColor White

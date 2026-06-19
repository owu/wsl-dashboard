param(
    [Parameter(Mandatory = $false)]
    [string]$Version,

    [Parameter(Mandatory = $false)]
    [switch]$Hash
)

# WSL Dashboard Installer Build Script

# Read version from Cargo.toml if not provided
if ([string]::IsNullOrWhiteSpace($Version)) {
    $cargoToml = Get-Content -Path "$PSScriptRoot/../../Cargo.toml" -Raw
    $versionMatch = [regex]::Match($cargoToml, 'version\s*=\s*"([^"]+)"')
    if (-not $versionMatch.Success) {
        Write-Host "Could not find version in Cargo.toml" -ForegroundColor Red
        exit 1
    }
    $VERSION = $versionMatch.Groups[1].Value
} else {
    $VERSION = $Version
}
$APP_NAME = "wsldashboard"

Write-Host "--- Starting Build Process for $APP_NAME v$VERSION ---" -ForegroundColor Cyan

# 1. Check for release binary
$EXE_PATH = Join-Path $PSScriptRoot "..\..\target\release\wsldashboard.exe"
if (-not (Test-Path $EXE_PATH)) {
    Write-Error "Release binary not found at $EXE_PATH"
    Write-Host "Step 1: Please run '.\build\portable\build.ps1' first to compile the project." -ForegroundColor Yellow
    exit 1
}
Write-Host "Step 1: Found release binary at $EXE_PATH" -ForegroundColor Gray

# 2. Check for Inno Setup (iscc.exe)
Write-Host "Step 2: Checking for Inno Setup Compiler (iscc.exe)..." -ForegroundColor Yellow
$ISCC = Get-Command iscc.exe -ErrorAction SilentlyContinue
if (-not $ISCC) {
    # Try common installation paths if not in PATH
    $Paths = @(
        "${env:ProgramFiles(x86)}\Inno Setup 6\ISCC.exe",
        "${env:ProgramFiles}\Inno Setup 6\ISCC.exe"
    )
    foreach ($Path in $Paths) {
        if (Test-Path $Path) {
            $ISCC = $Path
            break
        }
    }
}

if (-not $ISCC) {
    Write-Error "Inno Setup Compiler (iscc.exe) not found! Please install it from https://jrsoftware.org/isdl.php"
    exit 1
}

Write-Host "Found ISCC at: $ISCC" -ForegroundColor Gray

# 3. Build the installer
Write-Host "Step 3: Generating Installer..." -ForegroundColor Yellow
& $ISCC /DAppVersion=$VERSION "$PSScriptRoot\config.iss"
if ($LASTEXITCODE -ne 0) {
    Write-Error "Inno Setup build failed!"
    exit 1
}

$INSTALLER_PATH = Join-Path $PSScriptRoot "..\..\build\releases\WSLDashboard.$VERSION.Setup.x64.exe"
$ABS_PATH = Resolve-Path $INSTALLER_PATH

Write-Host "--- Success! Installer generated at: $ABS_PATH ---" -ForegroundColor Green

# Output SHA256 hash if -Hash flag is specified
if ($Hash) {
    $sha256 = (Get-FileHash -Path $ABS_PATH -Algorithm SHA256).Hash
    Write-Host "SHA256: $sha256" -ForegroundColor Cyan
}

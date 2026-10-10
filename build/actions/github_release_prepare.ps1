param (
    [Parameter(Mandatory=$true)] [string]$Version,
    [Parameter(Mandatory=$false)] [ValidateSet("x64","arm64")] [string]$Arch = "x64"
)

Write-Host "--------------------------------------------------"
Write-Host "[START] Preparing assets for version: $Version ($Arch)"
Write-Host "--------------------------------------------------"

# 1. Call Inno Setup to build the installer
Write-Host "[INFO] Building Installer using Inno Setup..."
try {
    .\build\setup\build.ps1 -Version "$Version" -Arch $Arch -ErrorAction Stop
    Write-Host "[OK] Installer build completed."
} catch {
    Write-Error "[ERROR] Failed to build installer: $_"
    exit 1
}

# 2. Prepare Portable Assets
Write-Host "`n[INFO] Preparing Portable assets..."
# x64 is built natively into target/release, arm64 is cross-compiled into its own target directory
if ($Arch -eq "arm64") {
    $release_dir = "target/aarch64-pc-windows-msvc/release"
} else {
    $release_dir = "target/release"
}
$raw_exe = "wsldashboard.exe"
$portable_exe = "WSLDashboard.$Version.Portable.$Arch.exe"
$portable_zip = "WSLDashboard.$Version.Portable.$Arch.zip"

if (Test-Path "$release_dir/$raw_exe") {
    Push-Location $release_dir
    try {
        # Rename to versioned name
        Move-Item "$raw_exe" "$portable_exe" -Force
        Write-Host "[OK] Renamed raw exe to $portable_exe"
        
        # Use 7z for maximum compression
        Write-Host "[INFO] Compressing Portable ZIP (7z mx9)..."
        7z a -tzip -mx9 "$portable_zip" "$portable_exe" | Out-Null
        Write-Host "[OK] Created: $portable_zip"
    } finally {
        Pop-Location
    }
} else {
    Write-Error "[ERROR] Portable exe not found at $release_dir/$raw_exe"
    Write-Host "[HINT] Run '.\build\portable\build.ps1 -Arch $Arch' first." -ForegroundColor Yellow
    exit 1
}

Write-Host "`n[SUCCESS] All assets are prepared and compressed."
Write-Host "--------------------------------------------------`n"

exit 0

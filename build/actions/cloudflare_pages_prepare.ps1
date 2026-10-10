param (
    [Parameter(Mandatory=$true)] [string]$Version,
    [Parameter(Mandatory=$true)] [string]$OutputDir,
    [Parameter(Mandatory=$true)] [string]$PortableX64Zip,
    [Parameter(Mandatory=$true)] [string]$SetupX64Exe,
    [Parameter(Mandatory=$true)] [string]$PortableArm64Zip,
    [Parameter(Mandatory=$true)] [string]$SetupArm64Exe
)

Write-Host "--------------------------------------------------"
Write-Host "[START] Starting build info generation for version: $Version"
Write-Host "[DIR] Output directory: $OutputDir"
Write-Host "--------------------------------------------------"

# 1. Ensure output directory exists
if (-not (Test-Path $OutputDir)) {
    Write-Host "[INFO] Creating output directory: $OutputDir"
    New-Item -ItemType Directory -Path $OutputDir -Force | Out-Null
} else {
    Write-Host "[INFO] Output directory already exists: $OutputDir"
}

# 2. Copy files
Write-Host "`n[INFO] Starting asset transfer..."
$filesToCopy = @(
    @{ Path = $PortableX64Zip; Name = "PortableX64Zip" },
    @{ Path = $SetupX64Exe; Name = "SetupX64Exe" },
    @{ Path = $PortableArm64Zip; Name = "PortableArm64Zip" },
    @{ Path = $SetupArm64Exe; Name = "SetupArm64Exe" }
)

foreach ($f in $filesToCopy) {
    if (Test-Path $f.Path) {
        try {
            $dest = Join-Path $OutputDir (Split-Path $f.Path -Leaf)
            Copy-Item -Path $f.Path -Destination $dest -Force -ErrorAction Stop
            Write-Host "[OK] $($f.Name) copied to: $dest"
        } catch {
            Write-Error "[ERROR] Failed to copy $($f.Name): $_"
            exit 1
        }
    } else {
        Write-Error "[ERROR] Missing critical asset: $($f.Name) at $($f.Path)"
        exit 1
    }
}

Write-Host "[SUCCESS] All assets copied successfully."
Write-Host "--------------------------------------------------`n"

exit 0

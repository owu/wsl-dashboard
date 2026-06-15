param (
    [Parameter(Mandatory=$true)] [string]$Version,
    [Parameter(Mandatory=$true)] [string]$OutputDir,
    [Parameter(Mandatory=$true)] [string]$PortableZip,
    [Parameter(Mandatory=$true)] [string]$SetupZip,
    [Parameter(Mandatory=$true)] [string]$SetupExe
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
    @{ Path = $PortableZip; Name = "PortableZip" },
    @{ Path = $SetupZip; Name = "SetupZip" },
    @{ Path = $SetupExe; Name = "SetupExe" }
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

# 3. Calculate hashes and prepare data
$dataList = @()

Write-Host "`n[INFO] Starting SHA256 checksum generation..."
function Get-FileInfoJson([string]$path, [string]$name) {
    if (Test-Path $path) {
        Write-Host "[INFO] Processing $name..."
        $hash = (Get-FileHash -Path $path -Algorithm SHA256).Hash.ToLower()
        Write-Host "[HASH] $name -> $hash"
        return @{ "file" = $name; "sha256" = "sha256:$hash" }
    } else {
        Write-Warning "[WARN] File not found for hashing: $name"
        return $null
    }
}

$fileInfos = @(
    @{ path = Join-Path $OutputDir (Split-Path $PortableZip -Leaf); name = "WSLDashboard.$Version.Portable.x64.zip" },
    @{ path = Join-Path $OutputDir (Split-Path $SetupExe -Leaf); name = "WSLDashboard.$Version.Setup.x64.exe" },
    @{ path = Join-Path $OutputDir (Split-Path $SetupZip -Leaf); name = "WSLDashboard.$Version.Setup.x64.zip" }
)

foreach ($f in $fileInfos) {
    $info = Get-FileInfoJson -path $f.path -name $f.name
    if ($info) { $dataList += $info }
}

# 4. Generate JSON string
Write-Host "`n[INFO] Formatting assets.json..."
$jsonContent = @"
{
    "err": 0,
    "msg": "success",
    "data": [
"@

for ($i = 0; $i -lt $dataList.Count; $i++) {
    $item = $dataList[$i]
    $entry = "        {`n"
    $entry += "        `"file`": `"$($item.file)`",`n"
    $entry += "        `"sha256`":`"$($item.sha256)`"`n"
    $entry += "        }"
    
    if ($i -lt $dataList.Count - 1) { $entry += "," }
    $jsonContent += "`n" + $entry
}
$jsonContent += "`n    ]`n}"

# 5. Write file (UTF-8 without BOM)
$Utf8NoBom = [System.Text.UTF8Encoding]::new($false)
[System.IO.File]::WriteAllText("$OutputDir/assets.json", $jsonContent, $Utf8NoBom)

Write-Host "[SUCCESS] assets.json generated with $($dataList.Count) file entries."
Write-Host "--------------------------------------------------`n"

exit 0

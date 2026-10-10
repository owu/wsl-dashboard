param (
    [Parameter(Mandatory = $true)] [string]$OutputDir,
    [Parameter(Mandatory = $false)] [string]$GitHubRepo = "owu/wsl-dashboard",
    [Parameter(Mandatory = $false)] [string]$DownloadUrl = "https://www.wslui.com/download/",
    [Parameter(Mandatory = $false)] [string]$Token = ""
)

$ErrorActionPreference = "Stop"

Write-Host "--------------------------------------------------"
Write-Host "[START] Generating Push Update Data from GitHub Releases"
Write-Host "Repo: $GitHubRepo"
Write-Host "Output Dir: $OutputDir"
Write-Host "--------------------------------------------------"

# 1. Create output directory structure
$ApiPath = Join-Path $OutputDir "common/v1/releases"
if (-not (Test-Path $ApiPath)) {
    New-Item -ItemType Directory -Path $ApiPath -Force | Out-Null
    Write-Host "[OK] Created directory: $ApiPath"
}

# 2. Fetch latest release info from GitHub API
$LatestUrl = "https://api.github.com/repos/$GitHubRepo/releases/latest"
Write-Host "[INFO] Fetching latest release from: $LatestUrl"

$RequestHeaders = @{ "User-Agent" = "wsl-dashboard-push-update" }
if (-not [string]::IsNullOrWhiteSpace($Token)) {
    $RequestHeaders["Authorization"] = "Bearer $Token"
    Write-Host "[INFO] Using authenticated request (Bearer token)"
}

try {
    $Release = Invoke-RestMethod -Uri $LatestUrl -Headers $RequestHeaders -Method Get -ErrorAction Stop
    Write-Host "[OK] Received release: $($Release.tag_name)"
} catch {
    Write-Error "[ERROR] Failed to fetch latest release: $_"
    exit 1
}

# 3. Extract version and release date
$Tag = $Release.tag_name
if ([string]::IsNullOrWhiteSpace($Tag) -or $Tag -notlike "v*") {
    Write-Error "[ERROR] Invalid tag name: '$Tag'. Must start with 'v'."
    exit 1
}
$Version = $Tag.Substring(1)

if ([string]::IsNullOrWhiteSpace($Release.published_at)) {
    Write-Error "[ERROR] Release has no published_at field."
    exit 1
}
# published_at is UTC; convert to UTC+8 to keep date consistent with the previous convention
$ReleaseDate = [DateTimeOffset]::Parse($Release.published_at).ToOffset([TimeSpan]::FromHours(8)).ToString("yyyy-MM-dd")
Write-Host "[OK] Version: $Version, Release Date: $ReleaseDate"

# 4. Build assets list from GitHub release assets
$Assets = @()
foreach ($Asset in $Release.assets) {
    if ([string]::IsNullOrWhiteSpace($Asset.name)) {
        continue
    }
    $Item = [ordered]@{
        file = $Asset.name
    }
    if (-not [string]::IsNullOrWhiteSpace($Asset.digest)) {
        $Item["sha256"] = $Asset.digest
    }
    $Assets += $Item
}
Write-Host "[OK] Assets count: $($Assets.Count)"

# 5. Generate 'latest' JSON file (no file extension on purpose)
$JsonData = [ordered]@{
    err  = 0
    msg  = "success"
    data = [ordered]@{
        version      = $Version
        release_date = $ReleaseDate
        download_url = $DownloadUrl
        assets       = $Assets
    }
}
$JsonString = $JsonData | ConvertTo-Json -Depth 10 -Compress

# Write file without extension (force UTF-8 without BOM using .NET API)
$Utf8NoBom = [System.Text.UTF8Encoding]::new($false)
$LatestFilePath = Join-Path $ApiPath "latest"
[System.IO.File]::WriteAllText($LatestFilePath, $JsonString, $Utf8NoBom)
Write-Host "[OK] Generated JSON at: $LatestFilePath"
Write-Host "[INFO] JSON Content: $JsonString"

# 6. Generate 'version' JSON file (no file extension on purpose, without download_url field)
$VersionData = [ordered]@{
    err  = 0
    msg  = "success"
    data = [ordered]@{
        version      = $Version
        release_date = $ReleaseDate
    }
}
$VersionJsonString = $VersionData | ConvertTo-Json -Depth 10 -Compress
$VersionFilePath = Join-Path $ApiPath "version"
[System.IO.File]::WriteAllText($VersionFilePath, $VersionJsonString, $Utf8NoBom)
Write-Host "[OK] Generated JSON at: $VersionFilePath"
Write-Host "[INFO] JSON Content: $VersionJsonString"

Write-Host "`n--------------------------------------------------"
Write-Host "[SUCCESS] Push Update Preparation Complete"
Write-Host "--------------------------------------------------`n"
exit 0

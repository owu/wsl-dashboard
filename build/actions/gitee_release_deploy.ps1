param (
    [Parameter(Mandatory=$true)] [string]$Version,
    [Parameter(Mandatory=$true)] [string]$DownloadDir,
    [Parameter(Mandatory=$true)] [string]$GiteeToken,
    [Parameter(Mandatory=$true)] [string]$RepoOwner,
    [Parameter(Mandatory=$true)] [string]$RepoName
)

$ApiBase = "https://gitee.com/api/v5"
$Tag = "v$Version"

Write-Host "--------------------------------------------------"
Write-Host "[START] Syncing assets to Gitee Release: $Tag"
Write-Host "[REPO]  $RepoOwner/$RepoName"
Write-Host "--------------------------------------------------"

# 1. Get or Create Release
$ReleaseId = $null
$GetUrl = "$ApiBase/repos/$RepoOwner/$RepoName/releases/tags/$Tag?access_token=$GiteeToken"

try {
    Write-Host "`n[INFO] Checking if Gitee Release $Tag exists..."
    $Response = Invoke-RestMethod -Uri $GetUrl -Method Get -ErrorAction Stop
    Write-Host "[DEBUG] Release Response: $($Response | ConvertTo-Json -Depth 3)"
    
    if ([string]::IsNullOrWhiteSpace($Response) -or $Response.id -eq $null) {
        Write-Host "[INFO] Release not found. Creating new release..."
        throw "Release not found"
    }
    
    $ReleaseId = $Response.id
    Write-Host "[OK] Release exists. ID: $ReleaseId"
} catch {
    Write-Host "[INFO] Release not found. Creating new release..."
    $PostUrl = "$ApiBase/repos/$RepoOwner/$RepoName/releases"
    $Body = @{
        access_token = $GiteeToken
        tag_name = $Tag
        name = $Tag
        body = "Auto-synced from GitHub Release $Tag"
        target_commitish = "main"
    }

    try {
        $Response = Invoke-RestMethod -Uri $PostUrl -Method Post -Body $Body -ErrorAction Stop
        Write-Host "[DEBUG] Created Release Response: $($Response | ConvertTo-Json -Depth 3)"
        $ReleaseId = $Response.id
        Write-Host "[OK] Created new release. ID: $ReleaseId"
    } catch {
        Write-Error "[ERROR] Failed to create Gitee Release: $_"
        exit 1
    }
}

# 2. Upload Assets
$UploadUrl = "$ApiBase/repos/$RepoOwner/$RepoName/releases/$ReleaseId/attach_files?access_token=$GiteeToken"

$filesToUpload = @(
    "WSLDashboard.$Version.Portable.x64.zip",
    "WSLDashboard.$Version.Setup.x64.exe",
    "WSLDashboard.$Version.Portable.arm64.zip",
    "WSLDashboard.$Version.Setup.arm64.exe"
)

Write-Host "`n[INFO] Starting asset upload..."
foreach ($fileName in $filesToUpload) {
    $filePath = Join-Path $DownloadDir $fileName
    if (-not (Test-Path $filePath)) {
        Write-Error "[ERROR] Asset not found locally: $filePath"
        exit 1
    }

    Write-Host "[INFO] Uploading $fileName to Gitee..."
    Write-Host "[DEBUG] Upload URL: ${UploadUrl}&name=$fileName"

    try {
        $Response = Invoke-RestMethod -Uri "${UploadUrl}&name=$fileName" -Method Post -Form @{ file = Get-Item -Path $filePath } -ErrorAction Stop
        Write-Host "[OK] Successfully uploaded $fileName"
    } catch {
        Write-Error "[ERROR] Failed to upload $($fileName): $_"
    }
}

Write-Host "`n--------------------------------------------------"
Write-Host "[SUCCESS] Gitee Release sync complete!"
Write-Host "--------------------------------------------------`n"

exit 0

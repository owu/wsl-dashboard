$isAdmin = ([Security.Principal.WindowsPrincipal][Security.Principal.WindowsIdentity]::GetCurrent()).IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)

if (-not $isAdmin) {
    Write-Host "Error: Please run this script as Administrator!" -ForegroundColor Red
    Write-Host "Press any key to exit..."
    $null = $Host.UI.RawUI.ReadKey("NoEcho,IncludeKeyDown")
    exit 1
}

$BackupDir = "D:\WSL-Backup"
$DefaultInstallPath = "D:\linux"

function Get-Distributions {
    $distributions = @()
    $rawOutput = & wsl.exe --list --quiet 2>&1
    $rawOutput | ForEach-Object {
        $line = $_ -replace '\x00', ''
        $trimmed = $line.Trim()
        if ($trimmed -ne "") {
            $distributions += $trimmed
        }
    }
    return $distributions
}

function Invoke-Delete {
    $distributions = Get-Distributions
    
    if ($distributions.Count -eq 0) {
        Write-Host "No WSL distributions found" -ForegroundColor Red
        return
    }
    
    Write-Host "The following distributions will be unregistered:" -ForegroundColor Yellow
    $distributions | ForEach-Object { Write-Host "  - $_" -ForegroundColor Red }
    
    Write-Host ""
    $confirm = Read-Host "Enter yes to confirm unregister all distributions (default: no)"
    
    if ($confirm -ne "yes") {
        Write-Host "Operation cancelled" -ForegroundColor Green
        return
    }
    
    Write-Host "`nUnregistering all distributions..." -ForegroundColor Yellow
    
    foreach ($distro in $distributions) {
        Write-Host "----------------------------------------" -ForegroundColor Cyan
        Write-Host "Unregistering: $distro" -ForegroundColor Red
        & wsl.exe --unregister $distro
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  Done" -ForegroundColor Green
        } else {
            Write-Host "  Failed! (Exit code: $LASTEXITCODE)" -ForegroundColor Red
        }
    }
    
    Write-Host "`nAll distributions unregistered!" -ForegroundColor Cyan
}

function Invoke-Export {
    if (-not (Test-Path $BackupDir)) {
        New-Item -ItemType Directory -Path $BackupDir -Force | Out-Null
    }
    
    Write-Host "Stopping all WSL distributions..." -ForegroundColor Yellow
    & wsl.exe --shutdown
    Start-Sleep -Seconds 2
    
    $distributions = Get-Distributions
    
    if ($distributions.Count -eq 0) {
        Write-Host "No WSL distributions found" -ForegroundColor Red
        return
    }
    
    Write-Host "Found distributions:" -ForegroundColor Cyan
    $distributions | ForEach-Object { Write-Host "  - $_" }
    
    foreach ($distro in $distributions) {
        Write-Host "----------------------------------------" -ForegroundColor Cyan
        $safeName = $distro -replace '[<>:"/\\|?*]', '_'
        $tarFile = Join-Path $BackupDir "$safeName.tar"
        
        Write-Host "Exporting: $distro -> $tarFile" -ForegroundColor Green
        
        & wsl.exe --export $distro $tarFile
        
        if ($LASTEXITCODE -eq 0) {
            $size = (Get-Item $tarFile).Length / 1GB
            Write-Host "  Done ($([math]::Round($size, 2)) GB)" -ForegroundColor Green
        } else {
            Write-Host "  Failed! (Exit code: $LASTEXITCODE)" -ForegroundColor Red
        }
    }
    
    Write-Host "`nExport completed!" -ForegroundColor Cyan
    Write-Host "Backup directory: $BackupDir"
}

function Invoke-Import {
    $wslconfigPath = Join-Path $env:USERPROFILE ".wslconfig"
    $enableSparseVhd = $false
    
    if (Test-Path $wslconfigPath) {
        $configContent = Get-Content $wslconfigPath -Raw
        if ($configContent -match 'sparseVhd\s*=\s*true') {
            $enableSparseVhd = $true
            Write-Host "Detected sparseVhd config, Sparse VHD mode will be enabled after import" -ForegroundColor Cyan
        }
    }
    
    if (-not (Test-Path $BackupDir)) {
        Write-Host "Backup directory not found: $BackupDir" -ForegroundColor Red
        return
    }
    
    Write-Host "Stopping all WSL distributions..." -ForegroundColor Yellow
    & wsl.exe --shutdown
    Start-Sleep -Seconds 2
    
    $existingDistros = Get-Distributions
    
    $tarFiles = Get-ChildItem -Path $BackupDir -Filter "*.tar" -File
    
    if ($tarFiles.Count -eq 0) {
        Write-Host "No tar files found: $BackupDir" -ForegroundColor Red
        return
    }
    
    Write-Host "Found tar files:" -ForegroundColor Cyan
    $tarFiles | ForEach-Object { Write-Host "  - $($_.Name)" }
    
    if (-not (Test-Path $DefaultInstallPath)) {
        New-Item -ItemType Directory -Path $DefaultInstallPath -Force | Out-Null
    }
    
    $importedDistros = @()
    $skippedDistros = @()
    
    foreach ($tarFile in $tarFiles) {
        $distroName = $tarFile.BaseName
        
        if ($existingDistros -contains $distroName) {
            Write-Host "Skipping: $distroName (already exists)" -ForegroundColor Yellow
            $skippedDistros += $distroName
            continue
        }
        
        $installPath = Join-Path $DefaultInstallPath $distroName
        
        if (-not (Test-Path $installPath)) {
            New-Item -ItemType Directory -Path $installPath -Force | Out-Null
        }

        Write-Host "----------------------------------------" -ForegroundColor Cyan
        Write-Host "Importing: $($tarFile.Name) -> $distroName" -ForegroundColor Green
        
        & wsl.exe --import $distroName $installPath $tarFile.FullName
        
        if ($LASTEXITCODE -eq 0) {
            Write-Host "  Done" -ForegroundColor Green
            $importedDistros += $distroName
        } else {
            Write-Host "  Failed! (Exit code: $LASTEXITCODE)" -ForegroundColor Red
        }
    }
    
    Write-Host "`nImport Results:" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    if ($importedDistros.Count -gt 0) {
        Write-Host "  Imported: $($importedDistros.Count)" -ForegroundColor Green
        $importedDistros | ForEach-Object { Write-Host "    - $_" }
    }
    if ($skippedDistros.Count -gt 0) {
        Write-Host "  Skipped (already exists): $($skippedDistros.Count)" -ForegroundColor Yellow
        $skippedDistros | ForEach-Object { Write-Host "    - $_" }
    }
    if ($importedDistros.Count -eq 0 -and $skippedDistros.Count -eq 0) {
        Write-Host "  No action taken" -ForegroundColor Gray
    }
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host "Install directory: $DefaultInstallPath"
    
    if ($enableSparseVhd -and $importedDistros.Count -gt 0) {
        Write-Host "`nEnabling Sparse VHD mode for newly imported distributions..." -ForegroundColor Yellow
        Write-Host "Stopping WSL..." -ForegroundColor Yellow
        & wsl.exe --shutdown
        Start-Sleep -Seconds 2
        
        foreach ($distroName in $importedDistros) {
            Write-Host "----------------------------------------" -ForegroundColor Cyan
            Write-Host "Setting: $distroName" -ForegroundColor Green
            & wsl.exe --manage $distroName --set-sparse true --allow-unsafe
            
            if ($LASTEXITCODE -eq 0) {
                Write-Host "  Done" -ForegroundColor Green
            } else {
                Write-Host "  Failed! (Exit code: $LASTEXITCODE)" -ForegroundColor Red
            }
        }
    }
}

while ($true) {
    Write-Host "`n========================================" -ForegroundColor Cyan
    Write-Host "       WSL Dashboard Toolkit" -ForegroundColor Cyan
    Write-Host "       https://www.wslui.com" -ForegroundColor Cyan
    Write-Host "========================================" -ForegroundColor Cyan
    Write-Host ""
    Write-Host "  Backup directory: $BackupDir" -ForegroundColor Gray
    Write-Host "  Install directory: $DefaultInstallPath" -ForegroundColor Gray
    Write-Host ""
    Write-Host "  Available commands:" -ForegroundColor Yellow
    Write-Host "    export  - Export all distributions" -ForegroundColor White
    Write-Host "    import  - Import all distributions" -ForegroundColor White
    Write-Host "    delete  - Unregister all distributions" -ForegroundColor White
    Write-Host "    exit    - Exit program" -ForegroundColor White
    Write-Host ""
    
    $action = Read-Host "Enter command"
    
    switch ($action.ToLower()) {
        "export" {
            Invoke-Export
        }
        "import" {
            Invoke-Import
        }
        "delete" {
            Invoke-Delete
        }
        "exit" {
            Write-Host "Goodbye!" -ForegroundColor Cyan
            exit 0
        }
        default {
            Write-Host "Invalid command: $action" -ForegroundColor Red
        }
    }
}
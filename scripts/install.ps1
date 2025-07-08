# Biome Installer Bootstrap Script for Windows (PowerShell)
# https://github.com/biomejs/installer

param(
    [Parameter(ValueFromRemainingArguments = $true)]
    [string[]]$InstallerArgs
)

$ErrorActionPreference = "Stop"

function Get-Architecture {
    switch ($env:PROCESSOR_ARCHITECTURE) {
        "AMD64" { return "x86_64" }
        "ARM64" { return "aarch64" }
        default { 
            Write-Error "Unsupported architecture: $env:PROCESSOR_ARCHITECTURE"
            exit 1
        }
    }
}

function Download-Installer {
    param([string[]]$Args)
    
    # Check if Invoke-WebRequest is available (should be in PowerShell 3.0+)
    if (-not (Get-Command Invoke-WebRequest -ErrorAction SilentlyContinue)) {
        Write-Error "Invoke-WebRequest is not available. Please upgrade PowerShell."
        exit 1
    }

    $arch = Get-Architecture
    $url = "https://github.com/biomejs/installer/releases/latest/download/biome-installer-windows-${arch}.exe"
    
    # Create temporary file
    $tempFile = [System.IO.Path]::GetTempFileName() + ".exe"
    
    try {
        Write-Host "Downloading installer from: $url"
        
        # Download the installer
        Invoke-WebRequest -Uri $url -OutFile $tempFile -UseBasicParsing
        
        # Execute the installer with passed arguments
        if ($Args) {
            & $tempFile $Args
        } else {
            & $tempFile
        }
    }
    catch {
        Write-Error "Failed to download or execute the installer: $_"
        exit 1
    }
    finally {
        # Clean up temporary file
        if (Test-Path $tempFile) {
            Remove-Item $tempFile -Force -ErrorAction SilentlyContinue
        }
    }
}

function Main {
    Download-Installer -Args $InstallerArgs
}

Main
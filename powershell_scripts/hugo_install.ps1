# Function to determine the OS and architecture
function Detect-OSArch {
    $archType = $env:PROCESSOR_ARCHITECTURE
    switch ($archType) {
        "AMD64"  { $archType = "amd64"; break }
        "ARM64"  { $archType = "arm64"; break }
        "IA64"   {
            Write-Host "Unsupported Itanium architecture"
            exit 1
        }
        "EM64T"  { $archType = "amd64"; break }  # Rare, but treated as amd64
        "X86"    {
            if ([Environment]::Is64BitOperatingSystem) {
                $archType = "amd64"
            } else {
                Write-Host "Unsupported 32-bit architecture"
                exit 1
            }
            break
        }
        default {
            Write-Host "Unsupported architecture"
            exit 1
        }
    }
    return "windows-$archType"
}

# Function to download Hugo
function Download-Hugo {
    param (
        [string]$version,
        [string]$isExtended
    )

    $osArch = Detect-OSArch
    $osType = $osArch.Split("-")[0]
    $archType = $osArch.Split("-")[1]

    $baseUrl = "https://github.com/gohugoio/hugo/releases/download/v$version"

    # Determine file extension and whether an extended version is needed
    $ext = "zip"
    $filePrefix = "hugo"
    if ($isExtended -eq "true") {
        $filePrefix = "hugo_extended"
    }

    # Construct download URL
    $downloadUrl = "$baseUrl/$filePrefix" + "_$version" + "_$osType-$archType.$ext"

    # Download and extract Hugo
    Write-Host "Downloading Hugo from $downloadUrl"
    Invoke-WebRequest -Uri $downloadUrl -OutFile "hugo.$ext"

    # Create a directory for Hugo if it doesn't exist
    $hugoPath = "$env:USERPROFILE\.local\bin"
    if (-Not (Test-Path $hugoPath)) {
        New-Item -Path $hugoPath -ItemType Directory
    }

    # Extract and move Hugo
    Expand-Archive -Path "hugo.$ext" -DestinationPath $hugoPath
    Move-Item -Path "$hugoPath\hugo.exe" -Destination $hugoPath

    # Remove artifacts
    Remove-Item -Path "hugo.$ext"

    Write-Host "Hugo downloaded, extracted, and moved to $hugoPath."
}

# Check for at least one argument (the version)
if ($args.Length -eq 0) {
    Write-Host "Usage: .\Download-Hugo.ps1 <version> [--extended]"
    exit 1
}

# Check for extended flag
$isExtended = "false"
if ($args[1] -eq "--extended") {
    $isExtended = "true"
}

Download-Hugo -version $args[0] -isExtended $isExtended

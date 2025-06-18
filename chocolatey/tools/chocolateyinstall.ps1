$ErrorActionPreference = 'Stop'

$packageName = 'ht-mcp'
$url64 = 'https://github.com/memextech/ht-mcp/releases/download/v0.1.0/ht-mcp-x86_64-pc-windows-msvc.exe'
$checksum64 = '$HASH_PLACEHOLDER'
$checksumType64 = 'sha256'

$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$installDir = Join-Path $toolsDir 'bin'

# Create bin directory if it doesn't exist
if (!(Test-Path $installDir)) {
    New-Item -ItemType Directory -Path $installDir -Force | Out-Null
}

$packageArgs = @{
    packageName   = $packageName
    unzipLocation = $installDir
    url64bit      = $url64
    checksum64    = $checksum64
    checksumType64= $checksumType64
    fileFullPath  = Join-Path $installDir 'ht-mcp.exe'
}

Get-ChocolateyWebFile @packageArgs

# Add to PATH if not already there
$binPath = $installDir
$envPath = [Environment]::GetEnvironmentVariable('PATH', 'Machine')
if ($envPath -notlike "*$binPath*") {
    [Environment]::SetEnvironmentVariable('PATH', "$envPath;$binPath", 'Machine')
    $env:PATH = "$env:PATH;$binPath"
}
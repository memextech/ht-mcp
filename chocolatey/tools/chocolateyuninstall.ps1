$ErrorActionPreference = 'Stop'

$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$installDir = Join-Path $toolsDir 'bin'

# Remove binary
$binaryPath = Join-Path $installDir 'ht-mcp.exe'
if (Test-Path $binaryPath) {
    Remove-Item $binaryPath -Force
}

# Remove from PATH
$binPath = $installDir
$envPath = [Environment]::GetEnvironmentVariable('PATH', 'Machine')
if ($envPath -like "*$binPath*") {
    $newPath = $envPath.Replace(";$binPath", "").Replace("$binPath;", "").Replace("$binPath", "")
    [Environment]::SetEnvironmentVariable('PATH', $newPath, 'Machine')
}
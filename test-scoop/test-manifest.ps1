# PowerShell script to test Scoop manifest locally
# Run this on Windows to validate the manifest

Write-Host "Testing Scoop manifest for ht-mcp..." -ForegroundColor Green

# Check if scoop is installed
if (!(Get-Command scoop -ErrorAction SilentlyContinue)) {
    Write-Host "❌ Scoop is not installed. Install it first:" -ForegroundColor Red
    Write-Host "Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser"
    Write-Host "irm get.scoop.sh | iex"
    exit 1
}

# Navigate to project root
$scriptPath = Split-Path -Parent $MyInvocation.MyCommand.Definition
$projectRoot = Split-Path -Parent $scriptPath
Set-Location $projectRoot

# Test 1: Validate JSON syntax
Write-Host "`n🔍 Testing JSON syntax..." -ForegroundColor Yellow
try {
    $manifest = Get-Content "scoop/ht-mcp.json" | ConvertFrom-Json
    Write-Host "✅ JSON syntax is valid" -ForegroundColor Green
} catch {
    Write-Host "❌ JSON syntax error: $($_.Exception.Message)" -ForegroundColor Red
    exit 1
}

# Test 2: Check required fields
Write-Host "`n🔍 Checking required fields..." -ForegroundColor Yellow
$requiredFields = @("version", "description", "homepage", "license", "architecture")
foreach ($field in $requiredFields) {
    if ($manifest.$field) {
        Write-Host "✅ $field is present" -ForegroundColor Green
    } else {
        Write-Host "❌ $field is missing" -ForegroundColor Red
    }
}

# Test 3: Validate architecture section
Write-Host "`n🔍 Checking architecture section..." -ForegroundColor Yellow
if ($manifest.architecture."64bit".url -and $manifest.architecture."64bit".bin) {
    Write-Host "✅ 64-bit architecture properly configured" -ForegroundColor Green
} else {
    Write-Host "❌ 64-bit architecture configuration incomplete" -ForegroundColor Red
}

# Test 4: Check autoupdate configuration
Write-Host "`n🔍 Checking autoupdate configuration..." -ForegroundColor Yellow
if ($manifest.checkver -eq "github" -and $manifest.autoupdate) {
    Write-Host "✅ Autoupdate properly configured" -ForegroundColor Green
} else {
    Write-Host "❌ Autoupdate configuration incomplete" -ForegroundColor Red
}

Write-Host "`n📋 Manifest Summary:" -ForegroundColor Cyan
Write-Host "Version: $($manifest.version)"
Write-Host "Description: $($manifest.description)"
Write-Host "Homepage: $($manifest.homepage)"
Write-Host "License: $($manifest.license)"
Write-Host "Download URL: $($manifest.architecture.'64bit'.url)"

Write-Host "`n🎯 Next steps:" -ForegroundColor Magenta
Write-Host "1. Update hash placeholder with actual release hash"
Write-Host "2. Test installation: scoop install .\scoop\ht-mcp.json"
Write-Host "3. Create test bucket for full integration test"
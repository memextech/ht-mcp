# Windows Installation Guide

HT-MCP supports multiple installation methods on Windows. Choose the method that best suits your needs.

## Quick Installation Methods

### Option 1: Scoop (Recommended for CLI users)

```powershell
# Install Scoop if you haven't already
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
irm get.scoop.sh | iex

# Install ht-mcp
scoop bucket add main
scoop install ht-mcp
```

**Advantages:**
- ✅ No admin privileges required
- ✅ Automatic PATH management
- ✅ Easy updates with `scoop update ht-mcp`
- ✅ Clean uninstall with `scoop uninstall ht-mcp`

### Option 2: Winget (Microsoft's official package manager)

```powershell
# Install ht-mcp
winget install MemexTech.HtMcp
```

**Advantages:**
- ✅ Built into Windows 11 and Windows 10 (with App Installer)
- ✅ Official Microsoft backing
- ✅ Automatic updates

### Option 3: Chocolatey

```powershell
# Install Chocolatey if you haven't already
Set-ExecutionPolicy Bypass -Scope Process -Force
[System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
iex ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

# Install ht-mcp
choco install ht-mcp
```

**Advantages:**
- ✅ Popular in enterprise environments
- ✅ Extensive package ecosystem
- ✅ Good Windows integration

## Manual Installation

### Option 4: Direct Binary Download

1. Go to the [latest release page](https://github.com/memextech/ht-mcp/releases/latest)
2. Download `ht-mcp-x86_64-pc-windows-msvc.exe`
3. Place it in a directory of your choice (e.g., `C:\Tools\ht-mcp\`)
4. Add the directory to your PATH:
   ```powershell
   # Add to PATH temporarily (current session only)
   $env:PATH += ";C:\Tools\ht-mcp"
   
   # Add to PATH permanently (all sessions)
   [Environment]::SetEnvironmentVariable("PATH", "$env:PATH;C:\Tools\ht-mcp", "User")
   ```
5. Rename the executable to `ht-mcp.exe` for convenience

## Verification

After installation, verify that ht-mcp is working:

```powershell
# Check installation
ht-mcp --version

# Test basic functionality
ht-mcp --help
```

## Platform Support

- **Architecture**: x64 (64-bit Intel/AMD)
- **OS Requirements**: Windows 10 1809+ or Windows 11
- **Dependencies**: None (statically linked)

## Troubleshooting

### Windows Defender SmartScreen Warning

If you see a SmartScreen warning when running the binary:
1. Click "More info"
2. Click "Run anyway"
3. This is normal for new, unsigned executables

### PowerShell Execution Policy

If you encounter execution policy errors:
```powershell
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

### PATH Issues

If `ht-mcp` command is not found after installation:
1. Restart your terminal/PowerShell
2. Check if the installation directory is in your PATH:
   ```powershell
   echo $env:PATH
   ```

## Updates

### Scoop
```powershell
scoop update ht-mcp
```

### Winget
```powershell
winget upgrade MemexTech.HtMcp
```

### Chocolatey
```powershell
choco upgrade ht-mcp
```

### Manual
Download the latest binary from the releases page and replace the existing one.

## Uninstallation

### Scoop
```powershell
scoop uninstall ht-mcp
```

### Winget
```powershell
winget uninstall MemexTech.HtMcp
```

### Chocolatey
```powershell
choco uninstall ht-mcp
```

### Manual
1. Delete the binary file
2. Remove the directory from your PATH environment variable

## Support

- 🐛 **Issues**: [GitHub Issues](https://github.com/memextech/ht-mcp/issues)
- 📖 **Documentation**: [Main README](../README.md)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/memextech/ht-mcp/discussions)
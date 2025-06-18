#!/bin/bash
set -euo pipefail

# Update package manifests with actual release hashes
# Usage: ./update-package-manifests.sh <version> <windows_hash>

VERSION=${1:-}
WINDOWS_HASH=${2:-}

if [[ -z "$VERSION" ]] || [[ -z "$WINDOWS_HASH" ]]; then
    echo "Usage: $0 <version> <windows_hash>"
    echo "Example: $0 v0.1.0 abc123def456..."
    exit 1
fi

# Clean version (remove 'v' prefix if present)
CLEAN_VERSION=${VERSION#v}

echo "Updating package manifests for version $CLEAN_VERSION with hash $WINDOWS_HASH"

# Update Scoop manifest
sed -i.bak "s/\"version\": \".*\"/\"version\": \"$CLEAN_VERSION\"/" scoop/ht-mcp.json
sed -i.bak "s|releases/download/v[^/]*/|releases/download/$VERSION/|g" scoop/ht-mcp.json
sed -i.bak "s/\$HASH_PLACEHOLDER/$WINDOWS_HASH/" scoop/ht-mcp.json

# Update Winget manifest
sed -i.bak "s/PackageVersion: .*/PackageVersion: $CLEAN_VERSION/" winget/ht-mcp.yaml
sed -i.bak "s|releases/download/v[^/]*/|releases/download/$VERSION/|g" winget/ht-mcp.yaml
sed -i.bak "s/\$HASH_PLACEHOLDER/$WINDOWS_HASH/" winget/ht-mcp.yaml

# Update Chocolatey manifest
sed -i.bak "s/<version>.*<\/version>/<version>$CLEAN_VERSION<\/version>/" chocolatey/ht-mcp.nuspec
sed -i.bak "s|releases/download/v[^/]*/'|releases/download/$VERSION/'|g" chocolatey/tools/chocolateyinstall.ps1
sed -i.bak "s/\$HASH_PLACEHOLDER/$WINDOWS_HASH/" chocolatey/tools/chocolateyinstall.ps1

# Clean up backup files
find . -name "*.bak" -delete

echo "✅ Package manifests updated successfully"
echo "Next steps:"
echo "1. Scoop: Submit PR to https://github.com/ScoopInstaller/Main"
echo "2. Winget: Submit PR to https://github.com/microsoft/winget-pkgs"
echo "3. Chocolatey: Run 'choco pack' and 'choco push' in chocolatey/ directory"
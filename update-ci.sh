#!/bin/bash

# Script to update CI configuration after PR is merged

# Make sure we're on main branch with latest changes
git checkout main
git pull

# Update existing CI
git mv .github/workflows/ci.yml .github/workflows/ci.yml.backup
git mv .github/workflows/consolidated-ci.yml .github/workflows/ci.yml
git rm .github/workflows/windows-support-ci.yml

# Commit the changes
git commit -m "Consolidate CI workflows with Windows support

- Replace existing CI with consolidated configuration
- Add Windows build target to main CI pipeline
- Remove separate Windows support CI workflow
- Maintain all CI best practices from both workflows

🤖 Generated with [Memex](https://memex.tech)
Co-Authored-By: Memex <noreply@memex.tech>"

# Push the changes
git push

echo "CI configuration updated successfully"
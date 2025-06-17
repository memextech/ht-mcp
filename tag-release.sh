#!/bin/bash
# Script to tag and push release after PR is merged

# Make sure we're on main branch with latest changes
git checkout main
git pull

# Create and push the tag
git tag -a v0.1.1 -m "Release version 0.1.1 with Windows platform support"
git push origin v0.1.1

echo "Tagged v0.1.1 and pushed to remote"
#!/bin/bash
set -e

# Run tests
echo "=== Running Tests ==="
./scripts/test.sh

# Build release
echo "=== Building Release ==="
./scripts/build.sh

# Extract version from Cargo.toml
VERSION=$(grep -E '^version[[:space:]]*=' Cargo.toml | head -n1 | cut -d'"' -f2 | cut -d"'" -f2)
if [ -z "$VERSION" ]; then
    VERSION="unknown"
fi

echo "=== Releasing version v$VERSION ==="
if [ -d .git ]; then
    git add .
    git commit -m "chore: release v$VERSION" || echo "No changes to commit"
    git tag -f -a "v$VERSION" -m "Release v$VERSION"
    echo "=== Pushing Git Tags ==="
    git push origin "v$VERSION"
else
    echo "Not a Git repository, skipping tagging/pushing."
fi

echo "=== Release Process Completed ==="

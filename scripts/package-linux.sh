#!/usr/bin/env bash
set -euo pipefail

TARGET="${1:?target required}"
ARTIFACT="${2:?artifact required}"
BIN="target/${TARGET}/release/steam-achievement-panel"

rm -rf dist
mkdir -p "dist/${ARTIFACT}"
cp "${BIN}" "dist/${ARTIFACT}/steam-achievement-panel"
chmod +x "dist/${ARTIFACT}/steam-achievement-panel"
printf '%s\n' 'Steam Achievement Panel' 'Run ./steam-achievement-panel after starting Steam.' > "dist/${ARTIFACT}/README.txt"
tar -czf "dist/${ARTIFACT}.tar.gz" -C dist "${ARTIFACT}"

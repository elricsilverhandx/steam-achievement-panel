#!/usr/bin/env bash
set -euo pipefail

TARGET="${1:?target required}"
ARTIFACT="${2:?artifact required}"
BIN="target/${TARGET}/release/steam-achievement-panel"
OUT_DIR="dist/${ARTIFACT}"

rm -rf dist
mkdir -p "${OUT_DIR}"
cp "${BIN}" "${OUT_DIR}/steam-achievement-panel"
chmod +x "${OUT_DIR}/steam-achievement-panel"

STEAM_SO="$(find "target/${TARGET}/release/build" -name libsteam_api.so -print -quit || true)"
if [ -n "${STEAM_SO}" ]; then
  cp "${STEAM_SO}" "${OUT_DIR}/libsteam_api.so"
fi

printf '%s\n' 'Steam Achievement Panel' 'Run ./steam-achievement-panel after starting Steam.' > "${OUT_DIR}/README.txt"
tar -czf "dist/${ARTIFACT}.tar.gz" -C dist "${ARTIFACT}"

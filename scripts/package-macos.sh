#!/usr/bin/env bash
set -euo pipefail

TARGET="${1:?target required}"
ARTIFACT="${2:?artifact required}"
APP_NAME="Steam Achievement Panel"
APP_BUNDLE="${APP_NAME}.app"
BIN="target/${TARGET}/release/steam-achievement-panel"

rm -rf dist
mkdir -p "dist/${APP_BUNDLE}/Contents/MacOS" "dist/${APP_BUNDLE}/Contents/Resources"
cp "${BIN}" "dist/${APP_BUNDLE}/Contents/MacOS/${APP_NAME}"
chmod +x "dist/${APP_BUNDLE}/Contents/MacOS/${APP_NAME}"

cat > "dist/${APP_BUNDLE}/Contents/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleDisplayName</key>
  <string>Steam Achievement Panel</string>
  <key>CFBundleExecutable</key>
  <string>Steam Achievement Panel</string>
  <key>CFBundleIdentifier</key>
  <string>com.elricsilverhandx.steam-achievement-panel</string>
  <key>CFBundleName</key>
  <string>Steam Achievement Panel</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>CFBundleShortVersionString</key>
  <string>0.1.0</string>
  <key>CFBundleVersion</key>
  <string>0.1.0</string>
  <key>LSMinimumSystemVersion</key>
  <string>11.0</string>
  <key>NSHighResolutionCapable</key>
  <true/>
</dict>
</plist>
PLIST

hdiutil create -volname "${APP_NAME}" -srcfolder "dist/${APP_BUNDLE}" -ov -format UDZO "dist/${ARTIFACT}.dmg"

# Steam Achievement Panel

A modern cross-platform Rust desktop app for Steam achievement inspection, achievement state changes, installed-game browsing, and simple idle-session tracking.

> ⚠️ **Important warning**
>
> This tool uses the local Steam client and Steamworks APIs. It can affect achievement/stat state for the selected AppID and may violate game, platform, multiplayer, VAC, anti-cheat, or community rules. Use only on your own account, only where you understand the risk, and avoid protected/online/multiplayer games. The app displays this warning before enabling write actions.

## Features

- Modern dark UI built with `egui`/`eframe`
- Local Steam library scanner using `libraryfolders.vdf` and `appmanifest_*.acf`
- Installed game list with Steam CDN header images
- Searchable game catalog
- Connect to a Steam AppID manually or by selecting a game
- Keep an initialized Steam session alive for idle/playtime-style tracking
- List achievements exposed by Steamworks
- Search, filter hidden, locked, and unlocked achievements
- Unlock or lock individual achievements
- Unlock all visible / lock all visible actions
- Clear warning gate before any write action
- Packaging scripts for:
  - macOS `.dmg`
  - Windows `.zip` with `.exe`
  - Linux `.tar.gz`

## Requirements

- Steam client must be running
- You must be logged in
- The selected AppID must be available to your account and compatible with Steamworks stats/achievement APIs
- Some games may block changes or protect achievements/stats

## Local development

```bash
cargo run
```

For quick Steamworks testing you can try AppID `480`.

## Release builds

A workflow template is included at:

```text
docs/release-workflow.yml
```

To enable GitHub Actions releases, copy it to:

```text
.github/workflows/release.yml
```

Then push a tag:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow template builds release artifacts and attaches them to a GitHub Release. It uses the scripts in `scripts/`.

## Packaging scripts

```bash
bash scripts/package-macos.sh aarch64-apple-darwin steam-achievement-panel-macos-arm64
bash scripts/package-linux.sh x86_64-unknown-linux-gnu steam-achievement-panel-linux-x64
```

On Windows:

```cmd
scripts\package-windows.cmd x86_64-pc-windows-msvc steam-achievement-panel-windows-x64
```

## Notes

The idle timer in the UI shows how long this helper process has kept a Steamworks session initialized for the selected AppID. Steam playtime counting behavior is controlled by Steam and the selected game; this app does not guarantee playtime credit.

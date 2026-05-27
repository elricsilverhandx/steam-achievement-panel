# Steam Achievement Panel

A modern cross-platform Rust desktop app for Steam achievement inspection, achievement state changes, and simple idle-session tracking.

> ⚠️ **Important warning**
>
> This tool uses the local Steam client and Steamworks APIs. It can affect achievement/stat state for the selected AppID and may violate game, platform, multiplayer, VAC, anti-cheat, or community rules. Use only on your own account, only where you understand the risk, and avoid protected/online/multiplayer games. The app displays this warning before enabling write actions.

## Features

- Modern dark UI built with `egui`/`eframe`
- Connect to a Steam AppID
- Keep an initialized Steam session alive for idle/playtime-style tracking
- List achievements exposed by Steamworks
- Unlock or lock individual achievements
- Unlock all visible / lock all visible actions
- Clear warning gate before any write action
- GitHub Actions release workflow for:
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

Push a tag like this:

```bash
git tag v0.1.0
git push origin v0.1.0
```

The workflow builds release artifacts and attaches them to a GitHub Release.

## Notes

The idle timer in the UI shows how long this helper process has kept a Steamworks session initialized for the selected AppID. Steam playtime counting behavior is controlled by Steam and the selected game; this app does not guarantee playtime credit.

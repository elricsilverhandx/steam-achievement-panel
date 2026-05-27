use anyhow::{Context, Result};
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, Default)]
pub struct SteamGame {
    pub app_id: u32,
    pub name: String,
    pub install_dir: String,
    pub library_path: PathBuf,
}

impl SteamGame {
    pub fn capsule_url(&self) -> String {
        format!("https://cdn.cloudflare.steamstatic.com/steam/apps/{}/header.jpg", self.app_id)
    }
}

pub fn scan_installed_games() -> Result<Vec<SteamGame>> {
    let steam_root = find_steam_root().context("Steam klasörü bulunamadı")?;
    let library_vdf = steam_root.join("steamapps").join("libraryfolders.vdf");
    let mut libraries = vec![steam_root.clone()];

    if let Ok(text) = fs::read_to_string(&library_vdf) {
        for path in extract_library_paths(&text) {
            if !libraries.iter().any(|existing| same_path(existing, &path)) {
                libraries.push(path);
            }
        }
    }

    let mut games = BTreeMap::<u32, SteamGame>::new();
    for library in libraries {
        let steamapps = library.join("steamapps");
        if !steamapps.exists() {
            continue;
        }

        for entry in fs::read_dir(&steamapps).with_context(|| format!("{} okunamadı", steamapps.display()))? {
            let entry = entry?;
            let path = entry.path();
            let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
                continue;
            };
            if !file_name.starts_with("appmanifest_") || !file_name.ends_with(".acf") {
                continue;
            }

            if let Ok(manifest) = fs::read_to_string(&path) {
                let values = parse_flat_vdf_values(&manifest);
                let Some(app_id) = values.get("appid").and_then(|value| value.parse::<u32>().ok()) else {
                    continue;
                };
                let name = values
                    .get("name")
                    .cloned()
                    .unwrap_or_else(|| format!("App {app_id}"));
                let install_dir = values.get("installdir").cloned().unwrap_or_default();

                games.entry(app_id).or_insert(SteamGame {
                    app_id,
                    name,
                    install_dir,
                    library_path: library.clone(),
                });
            }
        }
    }

    let mut games: Vec<_> = games.into_values().collect();
    games.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(games)
}

pub fn find_steam_root() -> Option<PathBuf> {
    if let Ok(path) = env::var("STEAM_DIR") {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(path);
        }
    }

    #[cfg(target_os = "windows")]
    {
        let candidates = [
            env::var("PROGRAMFILES(X86)").ok().map(|p| PathBuf::from(p).join("Steam")),
            env::var("PROGRAMFILES").ok().map(|p| PathBuf::from(p).join("Steam")),
        ];
        for candidate in candidates.into_iter().flatten() {
            if candidate.join("steamapps").exists() {
                return Some(candidate);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = home_dir() {
            let candidate = home.join("Library/Application Support/Steam");
            if candidate.join("steamapps").exists() {
                return Some(candidate);
            }
        }
    }

    #[cfg(target_os = "linux")]
    {
        if let Some(home) = home_dir() {
            let candidates = [
                home.join(".steam/steam"),
                home.join(".local/share/Steam"),
                home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam"),
            ];
            for candidate in candidates {
                if candidate.join("steamapps").exists() {
                    return Some(candidate);
                }
            }
        }
    }

    None
}

fn home_dir() -> Option<PathBuf> {
    env::var_os("HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("USERPROFILE").map(PathBuf::from))
}

fn extract_library_paths(text: &str) -> Vec<PathBuf> {
    let mut paths = Vec::new();
    let mut pending_path = false;

    for raw in text.lines() {
        let line = raw.trim();
        if line.starts_with("\"path\"") {
            let parts = quoted_parts(line);
            if parts.len() >= 2 {
                paths.push(PathBuf::from(parts[1].replace("\\\\", "\\")));
                pending_path = false;
            } else {
                pending_path = true;
            }
            continue;
        }

        if pending_path {
            let parts = quoted_parts(line);
            if let Some(value) = parts.first() {
                paths.push(PathBuf::from(value.replace("\\\\", "\\")));
            }
            pending_path = false;
        }
    }

    paths
}

fn parse_flat_vdf_values(text: &str) -> BTreeMap<String, String> {
    let mut values = BTreeMap::new();
    for raw in text.lines() {
        let parts = quoted_parts(raw.trim());
        if parts.len() >= 2 {
            values.insert(parts[0].to_lowercase(), parts[1].clone());
        }
    }
    values
}

fn quoted_parts(line: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_quote = false;
    let mut escaped = false;

    for ch in line.chars() {
        if escaped {
            current.push(ch);
            escaped = false;
            continue;
        }
        if ch == '\\' {
            escaped = true;
            continue;
        }
        if ch == '"' {
            if in_quote {
                parts.push(current.clone());
                current.clear();
            }
            in_quote = !in_quote;
            continue;
        }
        if in_quote {
            current.push(ch);
        }
    }

    parts
}

fn same_path(a: &Path, b: &Path) -> bool {
    let Ok(a) = a.canonicalize() else { return false; };
    let Ok(b) = b.canonicalize() else { return false; };
    a == b
}

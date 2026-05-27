use anyhow::{anyhow, Result};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use steamworks::{AppId, CallbackHandle, Client, UserStatsReceived, UserStatsStored};

#[derive(Clone, Debug)]
pub struct AchievementRow {
    pub api_name: String,
    pub display_name: String,
    pub description: String,
    pub hidden: bool,
    pub unlocked: bool,
    pub unlock_time: Option<u32>,
}

impl AchievementRow {
    pub fn visible_name(&self) -> &str {
        if self.display_name.trim().is_empty() {
            &self.api_name
        } else {
            &self.display_name
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct CallbackState {
    pub stats_received: bool,
    pub stats_ok: bool,
    pub last_store_ok: Option<bool>,
    pub last_callback_message: String,
}

pub struct SteamSession {
    pub app_id: u32,
    pub started_at: Instant,
    client: Client,
    callback_state: Arc<Mutex<CallbackState>>,
    _stats_received_handle: CallbackHandle,
    _stats_stored_handle: CallbackHandle,
}

impl SteamSession {
    pub fn connect(app_id: u32) -> Result<Self> {
        if app_id == 0 {
            return Err(anyhow!("AppID 0 is not valid."));
        }

        let client = Client::init_app(AppId(app_id)).map_err(|err| {
            anyhow!("Steam init failed: {err:?}. Start Steam, log in, and check that the AppID is available.")
        })?;

        let callback_state = Arc::new(Mutex::new(CallbackState::default()));

        let received_state = Arc::clone(&callback_state);
        let stats_received_handle = client.register_callback(move |value: UserStatsReceived| {
            if let Ok(mut state) = received_state.lock() {
                state.stats_received = true;
                state.stats_ok = value.result.is_ok();
                state.last_callback_message = if value.result.is_ok() {
                    "Steam stats callback ok.".to_owned()
                } else {
                    format!("Steam stats callback error: {:?}", value.result)
                };
            }
        });

        let stored_state = Arc::clone(&callback_state);
        let stats_stored_handle = client.register_callback(move |value: UserStatsStored| {
            if let Ok(mut state) = stored_state.lock() {
                state.last_store_ok = Some(value.result.is_ok());
                state.last_callback_message = if value.result.is_ok() {
                    "StoreStats ok.".to_owned()
                } else {
                    format!("StoreStats error: {:?}", value.result)
                };
            }
        });

        let session = Self {
            app_id,
            started_at: Instant::now(),
            client,
            callback_state,
            _stats_received_handle: stats_received_handle,
            _stats_stored_handle: stats_stored_handle,
        };

        session.request_stats();
        Ok(session)
    }

    pub fn elapsed(&self) -> Duration {
        self.started_at.elapsed()
    }

    pub fn callback_state(&self) -> CallbackState {
        self.callback_state
            .lock()
            .map(|state| state.clone())
            .unwrap_or_default()
    }

    pub fn run_callbacks(&self) {
        self.client.run_callbacks();
    }

    pub fn request_stats(&self) {
        let steam_id = self.client.user().steam_id().raw();
        self.client.user_stats().request_user_stats(steam_id);
    }

    pub fn is_logged_on(&self) -> bool {
        self.client.user().logged_on()
    }

    pub fn load_achievements(&self) -> Result<Vec<AchievementRow>> {
        self.request_stats();
        self.pump_callbacks_for(Duration::from_millis(700));

        let stats = self.client.user_stats();
        let names = stats
            .get_achievement_names()
            .ok_or_else(|| anyhow!("Could not load achievement names for this AppID."))?;

        let mut rows = Vec::with_capacity(names.len());
        for api_name in names {
            let achievement = stats.achievement(&api_name);
            let (unlocked, unlock_time) = achievement
                .get_achievement_and_unlock_time()
                .or_else(|_| achievement.get().map(|value| (value, 0)))
                .unwrap_or((false, 0));

            let display_name = achievement
                .get_achievement_display_attribute("name")
                .unwrap_or("")
                .to_owned();
            let description = achievement
                .get_achievement_display_attribute("desc")
                .unwrap_or("")
                .to_owned();
            let hidden = achievement
                .get_achievement_display_attribute("hidden")
                .map(|value| value == "1")
                .unwrap_or(false);

            rows.push(AchievementRow {
                api_name,
                display_name,
                description,
                hidden,
                unlocked,
                unlock_time: (unlocked && unlock_time > 0).then_some(unlock_time),
            });
        }

        rows.sort_by(|a, b| a.visible_name().to_lowercase().cmp(&b.visible_name().to_lowercase()));
        Ok(rows)
    }

    pub fn set_achievement(&self, api_name: &str, unlocked: bool) -> Result<()> {
        let stats = self.client.user_stats();
        let achievement = stats.achievement(api_name);

        if unlocked {
            achievement.set().map_err(|_| anyhow!("Could not unlock achievement: {api_name}"))?;
        } else {
            achievement.clear().map_err(|_| anyhow!("Could not lock achievement: {api_name}"))?;
        }

        stats.store_stats().map_err(|_| anyhow!("Steam StoreStats failed."))?;
        self.pump_callbacks_for(Duration::from_millis(900));
        Ok(())
    }

    pub fn set_many<'a>(&self, api_names: impl IntoIterator<Item = &'a str>, unlocked: bool) -> Result<usize> {
        let stats = self.client.user_stats();
        let mut changed = 0usize;

        for api_name in api_names {
            let achievement = stats.achievement(api_name);
            let result = if unlocked { achievement.set() } else { achievement.clear() };
            result.map_err(|_| anyhow!("Could not update achievement: {api_name}"))?;
            changed += 1;
        }

        if changed > 0 {
            stats.store_stats().map_err(|_| anyhow!("Steam StoreStats failed."))?;
            self.pump_callbacks_for(Duration::from_millis(1100));
        }

        Ok(changed)
    }

    fn pump_callbacks_for(&self, duration: Duration) {
        let start = Instant::now();
        while start.elapsed() < duration {
            self.client.run_callbacks();
            std::thread::sleep(Duration::from_millis(25));
        }
    }
}

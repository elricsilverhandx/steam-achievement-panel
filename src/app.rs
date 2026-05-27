use crate::steam_bridge::{AchievementRow, SteamSession};
use crate::steam_library::{scan_installed_games, SteamGame};
use crate::theme;
use eframe::egui::{self, Align, Button, CentralPanel, Color32, Frame, Layout, RichText, ScrollArea, Sense, SidePanel, Stroke, TopBottomPanel, Ui, Vec2};
use std::time::Duration;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ViewMode {
    Library,
    Achievements,
}

impl Default for ViewMode {
    fn default() -> Self {
        Self::Library
    }
}

pub struct AchievementPanelApp {
    app_id_input: String,
    search: String,
    game_search: String,
    status: String,
    session: Option<SteamSession>,
    achievements: Vec<AchievementRow>,
    games: Vec<SteamGame>,
    selected_game: Option<u32>,
    risk_accepted: bool,
    show_hidden: bool,
    only_locked: bool,
    only_unlocked: bool,
    view_mode: ViewMode,
}

impl Default for AchievementPanelApp {
    fn default() -> Self {
        let mut app = Self {
            app_id_input: "480".to_owned(),
            search: String::new(),
            game_search: String::new(),
            status: "Hazır. Önce Steam'i aç, sonra AppID bağlan veya kütüphaneyi tara.".to_owned(),
            session: None,
            achievements: Vec::new(),
            games: Vec::new(),
            selected_game: None,
            risk_accepted: false,
            show_hidden: true,
            only_locked: false,
            only_unlocked: false,
            view_mode: ViewMode::Library,
        };
        app.refresh_games();
        app
    }
}

impl eframe::App for AchievementPanelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if let Some(session) = &self.session {
            session.run_callbacks();
        }

        TopBottomPanel::top("top_bar")
            .frame(Frame::new().fill(theme::BG).inner_margin(egui::Margin::symmetric(18, 14)))
            .show(ctx, |ui| self.top_bar(ui));

        SidePanel::left("library_panel")
            .resizable(true)
            .default_width(320.0)
            .width_range(260.0..=430.0)
            .frame(Frame::new().fill(theme::PANEL).inner_margin(egui::Margin::same(14)))
            .show(ctx, |ui| self.library_panel(ui));

        CentralPanel::default()
            .frame(Frame::new().fill(theme::BG).inner_margin(egui::Margin::same(18)))
            .show(ctx, |ui| match self.view_mode {
                ViewMode::Library => self.library_home(ui),
                ViewMode::Achievements => self.achievement_panel(ui),
            });

        TopBottomPanel::bottom("status_bar")
            .frame(Frame::new().fill(Color32::from_rgb(7, 10, 18)).inner_margin(egui::Margin::symmetric(18, 10)))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(theme::muted("Durum:"));
                    ui.label(theme::accent(&self.status));
                    ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                        ui.label(theme::muted("Steam Achievement Panel • Rust/egui"));
                    });
                });
            });
    }
}

impl AchievementPanelApp {
    fn top_bar(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Steam Achievement Panel").size(28.0).strong().color(theme::TEXT));
                ui.label(theme::muted("Modern achievement manager + idle session tracker"));
            });

            ui.add_space(22.0);
            ui.separator();
            ui.add_space(12.0);

            ui.label(theme::muted("AppID"));
            ui.add_sized([110.0, 34.0], egui::TextEdit::singleline(&mut self.app_id_input).hint_text("480"));

            if ui.add(primary_button("Bağlan / Idle Başlat")).clicked() {
                self.connect_current_app();
            }

            if ui.add(soft_button("Achievementleri Yenile")).clicked() {
                self.reload_achievements();
            }

            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                self.session_badge(ui);
            });
        });
    }

    fn session_badge(&self, ui: &mut Ui) {
        let (text, color) = if let Some(session) = &self.session {
            let elapsed = format_duration(session.elapsed());
            let logged = if session.is_logged_on() { "online" } else { "offline" };
            (format!("App {} • {} • {}", session.app_id, elapsed, logged), theme::ACCENT_2)
        } else {
            ("Oturum yok".to_owned(), theme::MUTED)
        };

        Frame::new()
            .fill(theme::PANEL_SOFT)
            .stroke(Stroke::new(1.0, color.linear_multiply(0.45)))
            .corner_radius(egui::CornerRadius::same(255))
            .inner_margin(egui::Margin::symmetric(14, 8))
            .show(ui, |ui| {
                ui.label(RichText::new(text).color(color).strong());
            });
    }

    fn library_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.label(RichText::new("Kütüphane").size(20.0).strong().color(theme::TEXT));
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.add(small_button("Tara")).clicked() {
                    self.refresh_games();
                }
            });
        });

        ui.add_space(8.0);
        ui.add(egui::TextEdit::singleline(&mut self.game_search).hint_text("Oyun ara..."));
        ui.add_space(8.0);

        let count = self.filtered_games().len();
        ui.label(theme::muted(format!("{} kurulu oyun bulundu", count)));
        ui.add_space(8.0);

        ScrollArea::vertical().show(ui, |ui| {
            let games = self.filtered_games();
            for game in games {
                let selected = self.selected_game == Some(game.app_id);
                let response = game_row(ui, &game, selected);
                if response.clicked() {
                    self.selected_game = Some(game.app_id);
                    self.app_id_input = game.app_id.to_string();
                    self.view_mode = ViewMode::Achievements;
                    self.status = format!("{} seçildi. Bağlan düğmesiyle Steam oturumu başlatabilirsin.", game.name);
                }
            }
        });
    }

    fn library_home(&mut self, ui: &mut Ui) {
        hero_card(ui, |ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Oyunlarını seç, AppID ile bağlan, achievementleri yönet.").size(24.0).strong().color(theme::TEXT));
                ui.add_space(8.0);
                ui.label(theme::muted("Kurulu Steam oyunları yerel library manifestlerinden okunur. Oyun görselleri Steam CDN üzerinden gösterilir. Bir oyun seçtiğinde AppID otomatik dolar."));
            });
        });

        ui.add_space(14.0);
        warning_card(ui);

        ui.add_space(14.0);
        ui.horizontal(|ui| {
            metric_card(ui, "Kurulu oyun", self.games.len().to_string(), "Yerel Steam library taraması");
            let unlocked = self.achievements.iter().filter(|row| row.unlocked).count();
            metric_card(ui, "Açık başarım", unlocked.to_string(), "Son yüklenen AppID");
            let locked = self.achievements.iter().filter(|row| !row.unlocked).count();
            metric_card(ui, "Kilitli başarım", locked.to_string(), "Son yüklenen AppID");
        });

        ui.add_space(18.0);
        ui.label(RichText::new("Hızlı oyun seçimi").size(20.0).strong().color(theme::TEXT));
        ui.add_space(8.0);

        ScrollArea::vertical().show(ui, |ui| {
            let games = self.filtered_games();
            for chunk in games.chunks(2) {
                ui.horizontal(|ui| {
                    for game in chunk {
                        if game_card(ui, game).clicked() {
                            self.selected_game = Some(game.app_id);
                            self.app_id_input = game.app_id.to_string();
                            self.view_mode = ViewMode::Achievements;
                        }
                    }
                });
                ui.add_space(10.0);
            }
        });
    }

    fn achievement_panel(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(RichText::new("Achievement Panel").size(24.0).strong().color(theme::TEXT));
                ui.label(theme::muted("Bağlı AppID için başarımları listele, tek tek veya toplu yönet."));
            });
            ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                if ui.add(soft_button("Kütüphaneye Dön")).clicked() {
                    self.view_mode = ViewMode::Library;
                }
            });
        });

        ui.add_space(12.0);
        warning_card(ui);

        ui.add_space(12.0);
        Frame::new()
            .fill(theme::PANEL)
            .corner_radius(egui::CornerRadius::same(18))
            .inner_margin(egui::Margin::same(14))
            .show(ui, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.checkbox(&mut self.risk_accepted, RichText::new("Riskleri anladım, yazma işlemlerini etkinleştir.").color(theme::WARN).strong());
                    ui.separator();
                    ui.checkbox(&mut self.show_hidden, "Gizli başarımları göster");
                    ui.checkbox(&mut self.only_locked, "Sadece kilitli");
                    ui.checkbox(&mut self.only_unlocked, "Sadece açık");
                    if self.only_locked && self.only_unlocked {
                        self.only_unlocked = false;
                    }
                });

                ui.add_space(8.0);
                ui.horizontal(|ui| {
                    ui.add_sized([280.0, 34.0], egui::TextEdit::singleline(&mut self.search).hint_text("Achievement ara..."));
                    let can_write = self.can_write();
                    if ui.add_enabled(can_write, danger_button("Görünenleri Aç")).clicked() {
                        self.bulk_set_visible(true);
                    }
                    if ui.add_enabled(can_write, soft_button("Görünenleri Kilitle")).clicked() {
                        self.bulk_set_visible(false);
                    }
                });
            });

        ui.add_space(12.0);
        let visible = self.filtered_achievements();
        ui.label(theme::muted(format!("{} / {} başarım gösteriliyor", visible.len(), self.achievements.len())));
        ui.add_space(8.0);

        ScrollArea::vertical().show(ui, |ui| {
            for achievement in visible {
                let can_write = self.can_write();
                achievement_card(ui, &achievement, can_write, |target| {
                    self.set_one(&achievement.api_name, target);
                });
                ui.add_space(10.0);
            }
        });
    }

    fn refresh_games(&mut self) {
        match scan_installed_games() {
            Ok(games) => {
                self.status = format!("{} kurulu Steam oyunu bulundu.", games.len());
                self.games = games;
            }
            Err(err) => {
                self.status = format!("Steam kütüphanesi okunamadı: {err:#}");
            }
        }
    }

    fn connect_current_app(&mut self) {
        let Ok(app_id) = self.app_id_input.trim().parse::<u32>() else {
            self.status = "AppID sayı olmalı.".to_owned();
            return;
        };

        match SteamSession::connect(app_id) {
            Ok(session) => {
                self.status = format!("AppID {app_id} bağlandı. Idle oturumu başladı.");
                self.session = Some(session);
                self.reload_achievements();
                self.view_mode = ViewMode::Achievements;
            }
            Err(err) => {
                self.status = format!("Bağlanamadı: {err:#}");
            }
        }
    }

    fn reload_achievements(&mut self) {
        let Some(session) = &self.session else {
            self.status = "Önce bir AppID ile bağlan.".to_owned();
            return;
        };

        match session.load_achievements() {
            Ok(rows) => {
                let total = rows.len();
                let unlocked = rows.iter().filter(|row| row.unlocked).count();
                self.achievements = rows;
                self.status = format!("{total} başarım yüklendi, {unlocked} açık.");
            }
            Err(err) => {
                self.status = format!("Achievementler yüklenemedi: {err:#}");
            }
        }
    }

    fn can_write(&self) -> bool {
        self.session.is_some() && self.risk_accepted
    }

    fn set_one(&mut self, api_name: &str, unlocked: bool) {
        let Some(session) = &self.session else {
            self.status = "Oturum yok.".to_owned();
            return;
        };

        match session.set_achievement(api_name, unlocked) {
            Ok(()) => {
                self.status = if unlocked {
                    format!("{api_name} açıldı.")
                } else {
                    format!("{api_name} kilitlendi.")
                };
                self.reload_achievements();
            }
            Err(err) => {
                self.status = format!("İşlem başarısız: {err:#}");
            }
        }
    }

    fn bulk_set_visible(&mut self, unlocked: bool) {
        let Some(session) = &self.session else {
            self.status = "Oturum yok.".to_owned();
            return;
        };

        let names: Vec<String> = self.filtered_achievements().into_iter().map(|row| row.api_name).collect();
        match session.set_many(names.iter().map(String::as_str), unlocked) {
            Ok(count) => {
                self.status = if unlocked {
                    format!("{count} görünür başarım açıldı.")
                } else {
                    format!("{count} görünür başarım kilitlendi.")
                };
                self.reload_achievements();
            }
            Err(err) => self.status = format!("Toplu işlem başarısız: {err:#}"),
        }
    }

    fn filtered_games(&self) -> Vec<SteamGame> {
        let needle = self.game_search.trim().to_lowercase();
        self.games
            .iter()
            .filter(|game| {
                needle.is_empty()
                    || game.name.to_lowercase().contains(&needle)
                    || game.app_id.to_string().contains(&needle)
            })
            .cloned()
            .collect()
    }

    fn filtered_achievements(&self) -> Vec<AchievementRow> {
        let needle = self.search.trim().to_lowercase();
        self.achievements
            .iter()
            .filter(|row| self.show_hidden || !row.hidden)
            .filter(|row| !self.only_locked || !row.unlocked)
            .filter(|row| !self.only_unlocked || row.unlocked)
            .filter(|row| {
                needle.is_empty()
                    || row.api_name.to_lowercase().contains(&needle)
                    || row.display_name.to_lowercase().contains(&needle)
                    || row.description.to_lowercase().contains(&needle)
            })
            .cloned()
            .collect()
    }
}

fn hero_card(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    Frame::new()
        .fill(theme::PANEL)
        .stroke(Stroke::new(1.0, theme::ACCENT.linear_multiply(0.4)))
        .corner_radius(egui::CornerRadius::same(22))
        .inner_margin(egui::Margin::same(20))
        .show(ui, add_contents);
}

fn warning_card(ui: &mut Ui) {
    Frame::new()
        .fill(Color32::from_rgb(44, 32, 22))
        .stroke(Stroke::new(1.0, theme::WARN.linear_multiply(0.5)))
        .corner_radius(egui::CornerRadius::same(18))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.label(theme::warning("Uyarı: VAC/anti-cheat/multiplayer/protected oyunlarda kullanma."));
            ui.label(theme::muted("Bu uygulama seçilen AppID için Steamworks achievement/stat durumunu değiştirebilir. Kendi hesabında, riskini bildiğin oyunlarda kullan."));
        });
}

fn metric_card(ui: &mut Ui, title: &str, value: String, subtitle: &str) {
    Frame::new()
        .fill(theme::PANEL)
        .corner_radius(egui::CornerRadius::same(18))
        .inner_margin(egui::Margin::same(16))
        .show(ui, |ui| {
            ui.set_min_width(180.0);
            ui.label(theme::muted(title));
            ui.label(RichText::new(value).size(28.0).strong().color(theme::ACCENT_2));
            ui.label(theme::muted(subtitle));
        });
}

fn game_row(ui: &mut Ui, game: &SteamGame, selected: bool) -> egui::Response {
    let fill = if selected { theme::CARD_HOVER } else { theme::PANEL_SOFT };
    let inner = Frame::new()
        .fill(fill)
        .stroke(Stroke::new(1.0, if selected { theme::ACCENT } else { Color32::TRANSPARENT }))
        .corner_radius(egui::CornerRadius::same(14))
        .inner_margin(egui::Margin::same(10))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                ui.add(egui::Image::from_uri(game.capsule_url()).fit_to_exact_size(Vec2::new(92.0, 43.0)));
                ui.vertical(|ui| {
                    ui.label(RichText::new(&game.name).strong().color(theme::TEXT));
                    ui.label(theme::muted(format!("AppID {}", game.app_id)));
                });
            });
        });
    ui.interact(inner.response.rect, inner.response.id, Sense::click())
}

fn game_card(ui: &mut Ui, game: &SteamGame) -> egui::Response {
    let inner = Frame::new()
        .fill(theme::PANEL)
        .stroke(Stroke::new(1.0, theme::PANEL_SOFT))
        .corner_radius(egui::CornerRadius::same(20))
        .inner_margin(egui::Margin::same(12))
        .show(ui, |ui| {
            ui.set_min_width(330.0);
            ui.add(egui::Image::from_uri(game.capsule_url()).fit_to_exact_size(Vec2::new(300.0, 140.0)));
            ui.add_space(8.0);
            ui.label(RichText::new(&game.name).size(17.0).strong().color(theme::TEXT));
            ui.label(theme::muted(format!("AppID {} • {}", game.app_id, game.install_dir)));
        });
    ui.interact(inner.response.rect, inner.response.id, Sense::click())
}

fn achievement_card(ui: &mut Ui, row: &AchievementRow, can_write: bool, mut set_state: impl FnMut(bool)) {
    let border = if row.unlocked { theme::ACCENT_2 } else { theme::PANEL_SOFT };
    Frame::new()
        .fill(theme::PANEL)
        .stroke(Stroke::new(1.0, border.linear_multiply(0.65)))
        .corner_radius(egui::CornerRadius::same(18))
        .inner_margin(egui::Margin::same(14))
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let badge = if row.unlocked { "AÇIK" } else { "KİLİTLİ" };
                let badge_color = if row.unlocked { theme::ACCENT_2 } else { theme::MUTED };
                Frame::new()
                    .fill(badge_color.linear_multiply(0.18))
                    .corner_radius(egui::CornerRadius::same(255))
                    .inner_margin(egui::Margin::symmetric(10, 6))
                    .show(ui, |ui| ui.label(RichText::new(badge).color(badge_color).strong()));

                ui.vertical(|ui| {
                    ui.label(RichText::new(row.visible_name()).size(17.0).strong().color(theme::TEXT));
                    ui.label(theme::muted(&row.api_name));
                    if !row.description.trim().is_empty() {
                        ui.label(theme::muted(&row.description));
                    }
                    if row.hidden {
                        ui.label(theme::warning("Gizli başarım"));
                    }
                    if let Some(ts) = row.unlock_time {
                        ui.label(theme::muted(format!("Unlock time: {ts}")));
                    }
                });

                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    if row.unlocked {
                        if ui.add_enabled(can_write, soft_button("Kilitle")).clicked() {
                            set_state(false);
                        }
                    } else if ui.add_enabled(can_write, primary_button("Aç")).clicked() {
                        set_state(true);
                    }
                });
            });
        });
}

fn primary_button(text: &str) -> Button<'_> {
    Button::new(RichText::new(text).strong().color(Color32::WHITE)).fill(theme::ACCENT)
}

fn danger_button(text: &str) -> Button<'_> {
    Button::new(RichText::new(text).strong().color(Color32::WHITE)).fill(theme::DANGER)
}

fn soft_button(text: &str) -> Button<'_> {
    Button::new(RichText::new(text).color(theme::TEXT)).fill(theme::PANEL_SOFT)
}

fn small_button(text: &str) -> Button<'_> {
    Button::new(RichText::new(text).size(12.0).color(theme::TEXT)).fill(theme::CARD)
}

fn format_duration(duration: Duration) -> String {
    let secs = duration.as_secs();
    let h = secs / 3600;
    let m = (secs % 3600) / 60;
    let s = secs % 60;
    format!("{h:02}:{m:02}:{s:02}")
}

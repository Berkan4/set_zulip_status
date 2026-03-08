#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use eframe::egui;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::{Duration, Instant};

// -----------------------------
// Config parsing
// -----------------------------
#[derive(Debug)]
struct ZulipConfig {
    site: String,
    email: String,
    api_key: String,
}

fn parse_zuliprc(path: &PathBuf) -> Result<ZulipConfig, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Could not read config file: {e}"))?;

    let mut map: HashMap<String, String> = HashMap::new();
    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('#') || line.is_empty() || line.starts_with('[') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().to_string());
        }
    }

    let site = map.get("site").cloned().ok_or("Missing 'site' in config")?;
    let email = map.get("email").cloned().ok_or("Missing 'email' in config")?;
    let api_key = map.get("key").cloned().ok_or("Missing 'key' in config")?;

    Ok(ZulipConfig { site, email, api_key })
}

// -----------------------------
// Status options
// -----------------------------
struct StatusOption {
    label: &'static str,
    status_text: &'static str,
    emoji_name: &'static str,
    emoji_code: &'static str,
}

const STATUS_OPTIONS: &[StatusOption] = &[
    StatusOption {
        label: "Im Büro",
        status_text: "Im Büro",
        emoji_name: "office",
        emoji_code: "1f3e2",
    },
    StatusOption {
        label: "Arbeitet von zu Hause",
        status_text: "Arbeitet von zu Hause",
        emoji_name: "house",
        emoji_code: "1f3e0",
    },
];

// -----------------------------
// Zulip API response
// -----------------------------
#[derive(Deserialize)]
struct ZulipResponse {
    result: String,
    #[serde(default)]
    msg: String,
}

fn set_status(config: &ZulipConfig, option: &StatusOption) -> Result<(), String> {
    let client = Client::new();
    let url = format!("{}/api/v1/users/me/status", config.site.trim_end_matches('/'));

    let params = [
        ("status_text", option.status_text),
        ("away", "false"),
        ("emoji_name", option.emoji_name),
        ("emoji_code", option.emoji_code),
        ("reaction_type", "unicode_emoji"),
    ];

    let response = client
        .post(&url)
        .basic_auth(&config.email, Some(&config.api_key))
        .form(&params)
        .send()
        .map_err(|e| format!("Network error: {e}"))?;

    let body: ZulipResponse = response
        .json()
        .map_err(|e| format!("Failed to parse response: {e}"))?;

    if body.result == "success" {
        Ok(())
    } else {
        Err(format!("Zulip API error: {}", body.msg))
    }
}

// -----------------------------
// GUI App
// -----------------------------
enum AppState {
    Ready,
    Success,
    Error(String),
}

struct ZulipApp {
    config: Result<ZulipConfig, String>,
    state: AppState,
    success_time: Option<Instant>,
}

impl ZulipApp {
    fn new() -> Self {
        let path = dirs::home_dir()
            .unwrap_or_default()
            .join(".zuliprc/zuliprc");

        let config = parse_zuliprc(&path);
        Self {
            config,
            state: AppState::Ready,
            success_time: None,
        }
    }
}

impl eframe::App for ZulipApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Auto-close 2 seconds after success
        if let Some(t) = self.success_time {
            if t.elapsed() >= Duration::from_secs(2) {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            } else {
                // Keep repainting so we actually check the timer
                ctx.request_repaint_after(Duration::from_millis(100));
            }
        }

        // Apply a clean visual style
        let mut style = (*ctx.style()).clone();
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        style.spacing.item_spacing = egui::vec2(8.0, 10.0);
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                ui.add_space(20.0);

                ui.heading("🌐 Zulip Status");
                ui.add_space(4.0);
                ui.label("Wähle deinen Status aus:");
                ui.add_space(16.0);

                match &self.config {
                    Err(e) => {
                        let msg = e.clone();
                        ui.colored_label(
                            egui::Color32::RED,
                            format!("⚠ Konfigurationsfehler:\n{msg}"),
                        );
                    }
                    Ok(_) => {
                        match &self.state {
                            AppState::Success => {
                                let elapsed = self.success_time
                                    .map(|t| t.elapsed().as_secs())
                                    .unwrap_or(0);
                                let remaining = 2u64.saturating_sub(elapsed);
                                ui.colored_label(
                                    egui::Color32::GREEN,
                                    "✓ Status erfolgreich gesetzt!",
                                );
                                ui.add_space(6.0);
                                ui.label(format!("Fenster schließt in {remaining}s …"));
                            }
                            AppState::Error(err) => {
                                let err = err.clone();
                                ui.colored_label(
                                    egui::Color32::RED,
                                    format!("⚠ Fehler:\n{err}"),
                                );
                                ui.add_space(10.0);
                                if ui.button("↩ Zurück").clicked() {
                                    self.state = AppState::Ready;
                                }
                            }
                            AppState::Ready => {
                                for option in STATUS_OPTIONS {
                                    let btn = egui::Button::new(option.label)
                                        .min_size(egui::vec2(220.0, 36.0));
                                    if ui.add(btn).clicked() {
                                        if let Ok(cfg) = &self.config {
                                            match set_status(cfg, option) {
                                                Ok(()) => {
                                                    self.state = AppState::Success;
                                                    self.success_time = Some(Instant::now());
                                                }
                                                Err(e) => self.state = AppState::Error(e),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                ui.add_space(20.0);
            });
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Zulip Status setzen")
            .with_inner_size([320.0, 200.0])
            .with_resizable(false),
        ..Default::default()
    };

    eframe::run_native(
        "Zulip Status",
        options,
        Box::new(|_cc| Box::new(ZulipApp::new())),
    )
}

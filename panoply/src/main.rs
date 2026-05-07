#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod actions;
mod main_screen;
mod tools;

use eframe::egui;
use egui::{Color32, FontFamily, FontId, Stroke, Style, Visuals};
use main_screen::Screen;

pub const BG: Color32 = Color32::from_rgb(10, 10, 10);
pub const BG_WIDGET: Color32 = Color32::from_rgb(15, 15, 15);
pub const BG_WIDGET_HOVER: Color32 = Color32::from_rgb(25, 25, 25);
pub const BG_WIDGET_ACTIVE: Color32 = Color32::from_rgb(20, 20, 20);
pub const TEXT: Color32 = Color32::from_rgb(220, 220, 220);
pub const ACCENT: Color32 = Color32::from_rgb(0, 200, 70);
pub const BORDER: Color32 = Color32::from_rgb(50, 50, 50);
pub const BORDER_HOVER: Color32 = Color32::from_rgb(0, 200, 70);

pub const MAIN_SIZE: [f32; 2] = [480.0, 342.0];
pub const TOOL_SIZE: [f32; 2] = [480.0, 482.0];

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(MAIN_SIZE)
            .with_min_inner_size(MAIN_SIZE)
            .with_resizable(false)
            .with_active(true),
        ..Default::default()
    };

    eframe::run_native(
        "Panoply",
        options,
        Box::new(|cc| {
            configure_style(&cc.egui_ctx);
            Ok(Box::<App>::default())
        }),
    )
}

fn configure_style(ctx: &egui::Context) {
    let mut style = Style::default();
    style.visuals = Visuals::dark();
    style.visuals.override_text_color = Some(TEXT);
    style.visuals.panel_fill = BG;
    style.visuals.window_fill = BG;
    style.visuals.extreme_bg_color = Color32::from_rgb(5, 5, 5);
    style.visuals.faint_bg_color = Color32::from_rgb(18, 18, 18);
    style.visuals.popup_shadow = egui::epaint::Shadow::NONE;

    style.visuals.widgets.noninteractive.bg_fill = BG;
    style.visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0_f32, TEXT);

    style.visuals.widgets.inactive.bg_fill = BG_WIDGET;
    style.visuals.widgets.inactive.weak_bg_fill = BG_WIDGET;
    style.visuals.widgets.inactive.fg_stroke = Stroke::new(1.0_f32, TEXT);
    style.visuals.widgets.inactive.bg_stroke = Stroke::new(1.0_f32, BORDER);

    style.visuals.widgets.hovered.bg_fill = BG_WIDGET_HOVER;
    style.visuals.widgets.hovered.weak_bg_fill = BG_WIDGET_HOVER;
    style.visuals.widgets.hovered.fg_stroke = Stroke::new(1.0_f32, ACCENT);
    style.visuals.widgets.hovered.bg_stroke = Stroke::new(1.0_f32, BORDER_HOVER);

    style.visuals.widgets.active.bg_fill = BG_WIDGET_ACTIVE;
    style.visuals.widgets.active.weak_bg_fill = BG_WIDGET_ACTIVE;
    style.visuals.widgets.active.fg_stroke = Stroke::new(1.0_f32, ACCENT);
    style.visuals.widgets.active.bg_stroke = Stroke::new(1.5_f32, ACCENT);

    style.visuals.selection.bg_fill = Color32::from_rgb(0, 100, 40);
    style.visuals.selection.stroke = Stroke::new(1.5_f32, ACCENT);

    style.spacing.item_spacing = egui::vec2(8.0, 8.0);
    style.spacing.button_padding = egui::vec2(12.0, 10.0);

    style.text_styles = [
        (
            egui::TextStyle::Heading,
            FontId::new(22.0, FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Body,
            FontId::new(15.0, FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Monospace,
            FontId::new(14.0, FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Button,
            FontId::new(15.0, FontFamily::Monospace),
        ),
        (
            egui::TextStyle::Small,
            FontId::new(12.0, FontFamily::Monospace),
        ),
    ]
    .into();

    ctx.set_global_style(style);
}

fn resize(ctx: &egui::Context, size: [f32; 2]) {
    let v: egui::Vec2 = size.into();
    ctx.send_viewport_cmd(egui::ViewportCommand::MinInnerSize(v));
    ctx.send_viewport_cmd(egui::ViewportCommand::MaxInnerSize(v));
}

enum CurrentScreen {
    Main(main_screen::State),
    Base64Encode(tools::base64_encode::State),
    Base64Decode(tools::base64_decode::State),
    UrlEncode(tools::url_encode::State),
    UrlDecode(tools::url_decode::State),
    Uuid(tools::uuid::State),
    Password(tools::password::State),
}

struct App {
    current: CurrentScreen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            current: CurrentScreen::Main(main_screen::State::default()),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let mut next_screen: Option<CurrentScreen> = None;

        match &mut self.current {
            CurrentScreen::Main(state) => {
                let msgs = main_screen::view(state, ui);
                for msg in msgs {
                    match msg {
                        main_screen::Msg::OpenTool(screen) => {
                            resize(ui.ctx(), TOOL_SIZE);
                            next_screen = Some(match screen {
                                Screen::Main => CurrentScreen::Main(main_screen::State::default()),
                                Screen::Base64Encode => CurrentScreen::Base64Encode(
                                    tools::base64_encode::State::default(),
                                ),
                                Screen::Base64Decode => CurrentScreen::Base64Decode(
                                    tools::base64_decode::State::default(),
                                ),
                                Screen::UrlEncode => {
                                    CurrentScreen::UrlEncode(tools::url_encode::State::default())
                                }
                                Screen::UrlDecode => {
                                    CurrentScreen::UrlDecode(tools::url_decode::State::default())
                                }
                                Screen::UuidGenerate => {
                                    CurrentScreen::Uuid(tools::uuid::State::default())
                                }
                                Screen::PasswordGenerate => {
                                    CurrentScreen::Password(tools::password::State::default())
                                }
                            });
                        }
                        main_screen::Msg::CloseWindow => {
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                        _ => main_screen::update(&msg, state),
                    }
                }
            }
            CurrentScreen::Base64Encode(state) => {
                let msgs = tools::base64_encode::view(state, ui);
                for msg in msgs {
                    match msg {
                        tools::base64_encode::Msg::Back => {
                            resize(ui.ctx(), MAIN_SIZE);
                            next_screen = Some(CurrentScreen::Main(main_screen::State::default()));
                        }
                        _ => tools::base64_encode::update(&msg, state, ui.ctx()),
                    }
                }
            }
            CurrentScreen::Base64Decode(state) => {
                let msgs = tools::base64_decode::view(state, ui);
                for msg in msgs {
                    match msg {
                        tools::base64_decode::Msg::Back => {
                            resize(ui.ctx(), MAIN_SIZE);
                            next_screen = Some(CurrentScreen::Main(main_screen::State::default()));
                        }
                        _ => tools::base64_decode::update(&msg, state, ui.ctx()),
                    }
                }
            }
            CurrentScreen::UrlEncode(state) => {
                let msgs = tools::url_encode::view(state, ui);
                for msg in msgs {
                    match msg {
                        tools::url_encode::Msg::Back => {
                            resize(ui.ctx(), MAIN_SIZE);
                            next_screen = Some(CurrentScreen::Main(main_screen::State::default()));
                        }
                        _ => tools::url_encode::update(&msg, state, ui.ctx()),
                    }
                }
            }
            CurrentScreen::UrlDecode(state) => {
                let msgs = tools::url_decode::view(state, ui);
                for msg in msgs {
                    match msg {
                        tools::url_decode::Msg::Back => {
                            resize(ui.ctx(), MAIN_SIZE);
                            next_screen = Some(CurrentScreen::Main(main_screen::State::default()));
                        }
                        _ => tools::url_decode::update(&msg, state, ui.ctx()),
                    }
                }
            }
            CurrentScreen::Uuid(state) => {
                let msgs = tools::uuid::view(state, ui);
                for msg in msgs {
                    match msg {
                        tools::uuid::Msg::Back => {
                            resize(ui.ctx(), MAIN_SIZE);
                            next_screen = Some(CurrentScreen::Main(main_screen::State::default()));
                        }
                        _ => tools::uuid::update(&msg, state, ui.ctx()),
                    }
                }
            }
            CurrentScreen::Password(state) => {
                let msgs = tools::password::view(state, ui);
                for msg in msgs {
                    match msg {
                        tools::password::Msg::Back => {
                            resize(ui.ctx(), MAIN_SIZE);
                            next_screen = Some(CurrentScreen::Main(main_screen::State::default()));
                        }
                        _ => tools::password::update(&msg, state, ui.ctx()),
                    }
                }
            }
        }

        if let Some(next) = next_screen {
            self.current = next;
        }
    }
}

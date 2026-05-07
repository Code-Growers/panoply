use eframe::egui;
use egui::{CentralPanel, ScrollArea, Stroke, TextEdit};
use crate::{ACCENT, BG_WIDGET, BORDER};

#[derive(Clone, Default, PartialEq)]
pub enum Screen {
    #[default]
    Main,
    Base64Encode,
    Base64Decode,
    UrlEncode,
    UrlDecode,
    UuidGenerate,
    PasswordGenerate,
}

pub struct ToolDef {
    pub name: &'static str,
    pub screen: Screen,
}

pub const TOOLS: &[ToolDef] = &[
    ToolDef { name: "Base64 Encode", screen: Screen::Base64Encode },
    ToolDef { name: "Base64 Decode", screen: Screen::Base64Decode },
    ToolDef { name: "URL Encode", screen: Screen::UrlEncode },
    ToolDef { name: "URL Decode", screen: Screen::UrlDecode },
    ToolDef { name: "UUID Generate", screen: Screen::UuidGenerate },
    ToolDef { name: "Password Generate", screen: Screen::PasswordGenerate },
];

pub struct State {
    pub search: String,
    pub prev_search: String,
    pub selected_index: usize,
}

impl Default for State {
    fn default() -> Self {
        Self {
            search: String::new(),
            prev_search: String::new(),
            selected_index: 0,
        }
    }
}

#[derive(Clone)]
pub enum Msg {
    SearchChanged,
    SelectPrevious,
    SelectNext,
    OpenTool(Screen),
    CloseWindow,
}

pub fn update(msg: &Msg, state: &mut State) {
    match msg {
        Msg::SearchChanged => {
            state.selected_index = 0;
            state.prev_search.clone_from(&state.search);
        }
        Msg::SelectPrevious => {
            if state.selected_index > 0 {
                state.selected_index -= 1;
            }
        }
        Msg::SelectNext => {
            let query = state.search.to_lowercase();
            let count = TOOLS
                .iter()
                .filter(|t| t.name.to_lowercase().contains(&query))
                .count();
            if state.selected_index + 1 < count {
                state.selected_index += 1;
            }
        }
        _ => {}
    }
}

pub fn view(state: &mut State, ui: &mut egui::Ui) -> Vec<Msg> {
    let mut msgs = Vec::new();

    CentralPanel::default().show_inside(ui, |ui| {
        ui.add_space(8.0);
        let search_response = ui.add(
            TextEdit::singleline(&mut state.search)
                .hint_text("Search tools...")
                .desired_width(f32::INFINITY)
                .margin(egui::vec2(10.0, 10.0)),
        );
        search_response.request_focus();
        ui.add_space(8.0);

        let query = state.search.to_lowercase();
        let filtered: Vec<usize> = TOOLS
            .iter()
            .enumerate()
            .filter(|(_, t)| t.name.to_lowercase().contains(&query))
            .map(|(i, _)| i)
            .collect();

        if state.search != state.prev_search {
            msgs.push(Msg::SearchChanged);
        }

        if !filtered.is_empty() {
            if state.selected_index >= filtered.len() {
                state.selected_index = filtered.len() - 1;
            }

            if ui.input(|i| i.key_pressed(egui::Key::ArrowUp)) && state.selected_index > 0 {
                msgs.push(Msg::SelectPrevious);
            }
            if ui.input(|i| i.key_pressed(egui::Key::ArrowDown))
                && state.selected_index < filtered.len() - 1
            {
                msgs.push(Msg::SelectNext);
            }
            if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                msgs.push(Msg::CloseWindow);
            }
            if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                let tool = &TOOLS[filtered[state.selected_index]];
                msgs.push(Msg::OpenTool(tool.screen.clone()));
            }
        } else {
            state.selected_index = 0;
        }

        ScrollArea::vertical().show(ui, |ui| {
            for (vis_idx, tool_idx) in filtered.iter().enumerate() {
                let tool = &TOOLS[*tool_idx];
                let is_selected = vis_idx == state.selected_index;
                let btn = if is_selected {
                    egui::Button::new(egui::RichText::new(tool.name).color(ACCENT))
                        .fill(BG_WIDGET)
                        .stroke(Stroke::new(1.5_f32, ACCENT))
                } else {
                    egui::Button::new(tool.name)
                        .fill(BG_WIDGET)
                        .stroke(Stroke::new(1.0_f32, BORDER))
                };

                let response = ui.add_sized([ui.available_width(), 0.0], btn);

                if response.clicked() {
                    msgs.push(Msg::OpenTool(tool.screen.clone()));
                }
            }
        });
    });

    msgs
}

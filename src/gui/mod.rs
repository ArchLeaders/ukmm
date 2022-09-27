mod info;
mod mods;
mod picker;
mod visuals;
use crate::{core::Manager, logger::Entry, mods::Mod};
use eframe::{
    egui::{FontData, FontDefinitions},
    epaint::FontFamily,
    NativeOptions,
};
use egui::{
    self, style::Margin, text::LayoutJob, Color32, ComboBox, FontId, Frame, Label, Sense,
    TextFormat, Ui,
};
use flume::{Receiver, Sender};
use im::Vector;
use join_str::jstr;
use std::{collections::VecDeque, path::PathBuf, sync::Arc};

use self::picker::FilePickerState;

// #[inline(always)]
// fn common_frame() -> Frame {
//     Frame {
//         stroke: Stroke::new(0.1, Color32::DARK_GRAY),
//         inner_margin: Margin::same(4.),
//         fill: visuals::panel(),
//         ..Default::default()
//     }
// }

pub enum Message {
    Log(Entry),
    SelectOnly(usize),
    SelectAlso(usize),
    Deselect(usize),
    ClearSelect,
    StartDrag(usize),
    ClearDrag,
    SetDragDest(usize),
    MoveSelected(usize),
    FilePickerUp,
    FilePickerBack,
    FilePickerSet(Option<PathBuf>),
    ChangeProfile(String),
}

enum Tabs {
    Info,
    Install,
}

enum FocusedPane {}

struct App {
    core: Arc<Manager>,
    channel: (Sender<Message>, Receiver<Message>),
    mods: Vector<Mod>,
    selected: Vector<Mod>,
    drag_index: Option<usize>,
    hover_index: Option<usize>,
    picker_state: FilePickerState,
    tab: Tabs,
    logs: VecDeque<Entry>,
}

impl App {
    fn new(cc: &eframe::CreationContext) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "Rodin".to_owned(),
            FontData::from_static(include_bytes!("../../assets/rodin.otf")),
        );
        fonts.font_data.insert(
            "RodinBold".to_owned(),
            FontData::from_static(include_bytes!("../../assets/rodin-bold.otf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "Rodin".to_owned());
        fonts.families.insert(
            FontFamily::Name("Bold".into()),
            vec!["RodinBold".to_owned()],
        );
        cc.egui_ctx.set_fonts(fonts);
        cc.egui_ctx.set_pixels_per_point(1.);
        let core = Arc::new(Manager::init().unwrap());
        let mods: Vector<_> = core.mod_manager().all_mods().map(|m| m.clone()).collect();
        let (send, recv) = flume::unbounded();
        crate::logger::LOGGER.set_sender(send.clone());
        crate::logger::LOGGER.flush_queue();
        log::info!("Logger initialized");
        Self {
            channel: (send, recv),
            selected: mods.front().cloned().into_iter().collect(),
            drag_index: None,
            hover_index: None,
            picker_state: Default::default(),
            mods,
            core,
            logs: VecDeque::new(),
            tab: Tabs::Info,
        }
    }

    fn do_update(&self, message: Message) {
        self.channel.0.send(message).unwrap();
    }

    fn handle_update(&mut self, ctx: &eframe::egui::Context) {
        if let Ok(msg) = self.channel.1.try_recv() {
            match msg {
                Message::Log(entry) => {
                    self.logs.push_back(entry);
                    ctx.request_repaint();
                }
                Message::SelectOnly(i) => {
                    let index = i.clamp(0, self.mods.len() - 1);
                    let mod_ = &self.mods[index];
                    if self.selected.contains(mod_) {
                        self.selected.retain(|m| m == mod_);
                    } else {
                        self.selected.clear();
                        self.selected.push_back(self.mods[index].clone());
                    }
                    self.drag_index = None;
                }
                Message::SelectAlso(i) => {
                    let index = i.clamp(0, self.mods.len() - 1);
                    let mod_ = &self.mods[index];
                    if !self.selected.contains(mod_) {
                        self.selected.push_back(mod_.clone());
                    }
                    self.drag_index = None;
                }
                Message::Deselect(i) => {
                    let index = i.clamp(0, self.mods.len() - 1);
                    let mod_ = &self.mods[index];
                    self.selected.retain(|m| m != mod_);
                    self.drag_index = None;
                }
                Message::ClearSelect => {
                    self.selected.clear();
                    self.drag_index = None;
                }
                Message::StartDrag(i) => {
                    if ctx.input().pointer.any_released() {
                        self.drag_index = None;
                    }
                    self.drag_index = Some(i);
                    let mod_ = &self.mods[i];
                    if !self.selected.contains(mod_) {
                        if !ctx.input().modifiers.ctrl {
                            self.selected.clear();
                        }
                        self.selected.push_back(mod_.clone());
                    }
                }
                Message::ClearDrag => {
                    self.drag_index = None;
                }
                Message::SetDragDest(i) => self.hover_index = Some(i),
                Message::MoveSelected(dest_index) => {
                    if self.selected.len() == self.mods.len() {
                        return;
                    }
                    self.mods.retain(|m| !self.selected.contains(m));
                    for (i, selected_mod) in self.selected.iter().enumerate() {
                        self.mods.insert(dest_index + i, selected_mod.clone());
                    }
                    self.hover_index = None;
                    self.drag_index = None;
                }
                Message::FilePickerUp => {
                    let has_parent = self.picker_state.path.parent().is_some();
                    if has_parent {
                        self.picker_state
                            .history
                            .push(self.picker_state.path.clone());
                        self.picker_state
                            .set_path(self.picker_state.path.parent().unwrap().to_path_buf());
                    }
                }
                Message::FilePickerBack => {
                    if let Some(prev) = self.picker_state.history.pop() {
                        self.picker_state.set_path(prev);
                    }
                }
                Message::FilePickerSet(path) => {
                    self.picker_state
                        .history
                        .push(self.picker_state.path.clone());
                    let path = match path {
                        Some(path) => path,
                        None => self.picker_state.path_input.as_str().into(),
                    };
                    self.picker_state.set_path(path);
                }
                Message::ChangeProfile(profile) => {
                    todo!("Change profile");
                }
            }
        }
    }

    fn render_menu(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::top("menu_bar").show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.menu_button("File", Self::file_menu);
                ui.menu_button("Edit", Self::edit_menu);
            });
        });
    }

    fn file_menu(ui: &mut Ui) {
        if ui.button("Open mod…").clicked() {
            // todo!("Open mod");
        }
    }

    fn edit_menu(ui: &mut Ui) {
        if ui.button("Settings").clicked() {
            todo!("Settings");
        }
    }

    fn render_profile_menu(&mut self, ui: &mut Ui) {
        ui.horizontal(|ui| {
            let current_profile = self
                .core
                .settings()
                .platform_config()
                .map(|c| c.profile.to_string())
                .unwrap_or_else(|| "Default".to_owned());
            ComboBox::from_id_source("profiles")
                .selected_text(&current_profile)
                .show_ui(ui, |ui| {
                    self.core.settings().profiles().for_each(|profile| {
                        if ui
                            .selectable_label(profile.as_str() == current_profile, profile.as_str())
                            .clicked()
                        {
                            self.do_update(Message::ChangeProfile(profile.into()));
                        }
                    });
                })
                .response
                .on_hover_text("Select Mod Profile");
            ui.button("🗑").on_hover_text("Delete Profile");
            ui.button("✚").on_hover_text("New Profile");
            ui.button("☰").on_hover_text("Manage Profiles…");
            ui.label(format!(
                "{} Mods / {} Active",
                self.mods.len(),
                self.mods.iter().filter(|m| m.enabled).count()
            ));
        });
    }

    fn render_log(&self, ctx: &egui::Context) {
        egui::TopBottomPanel::bottom("log")
            .resizable(true)
            .show(ctx, |ui| {
                egui::ScrollArea::both()
                    .auto_shrink([false, false])
                    .stick_to_bottom(true)
                    .show(ui, |ui| {
                        let mut job = LayoutJob::default();
                        self.logs.iter().for_each(|entry| {
                            job.append(
                                &jstr!("[{&entry.timestamp}] "),
                                0.,
                                TextFormat {
                                    color: Color32::GRAY,
                                    font_id: FontId::monospace(10.),
                                    ..Default::default()
                                },
                            );
                            job.append(
                                &jstr!("{&entry.level} "),
                                0.,
                                TextFormat {
                                    color: match entry.level.as_str() {
                                        "INFO" => visuals::GREEN,
                                        "WARN" => visuals::ORGANGE,
                                        "ERROR" => visuals::RED,
                                        "DEBUG" => visuals::BLUE,
                                        _ => visuals::YELLOW,
                                    },
                                    font_id: FontId::monospace(10.),
                                    ..Default::default()
                                },
                            );
                            job.append(
                                &entry.args,
                                1.,
                                TextFormat {
                                    color: Color32::WHITE,
                                    font_id: FontId::monospace(10.),
                                    ..Default::default()
                                },
                            );
                            job.append("\n", 0.0, Default::default());
                        });
                        let text = job.text.clone();
                        if ui
                            .add(Label::new(job).sense(Sense::click()))
                            .on_hover_text("Click to copy")
                            .clicked()
                        {
                            ui.output().copied_text = text;
                        }
                    });
            });
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, frame: &mut eframe::Frame) {
        self.handle_update(ctx);
        self.render_menu(ctx);
        let mut max_width = 0.;
        egui::SidePanel::right("right_panel")
            .resizable(true)
            .max_width(ctx.used_size().x / 3.)
            .min_width(0.)
            .show(ctx, |ui| {
                max_width = ui.available_width();
                egui::ScrollArea::vertical()
                    .id_source("right_panel_scroll")
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            if ui
                                .selectable_label(matches!(self.tab, Tabs::Info), "Mod Info")
                                .clicked()
                            {
                                self.tab = Tabs::Info;
                            }
                            if ui
                                .selectable_label(matches!(self.tab, Tabs::Install), "Install")
                                .clicked()
                            {
                                self.tab = Tabs::Install;
                            }
                        });
                        match self.tab {
                            Tabs::Info => {
                                if let Some(mod_) = self.selected.front() {
                                    info::render_mod_info(mod_, ui);
                                } else {
                                    ui.centered_and_justified(|ui| {
                                        ui.label("No mod selected");
                                    });
                                }
                            }
                            Tabs::Install => {
                                self.render_file_picker(ui);
                            }
                        }
                    });
                ui.allocate_space(ui.available_size());
                max_width -= ui.min_rect().width();
            });
        egui::CentralPanel::default().show(ctx, |ui| {
            self.render_profile_menu(ui);
            egui::ScrollArea::both()
                .id_source("mod_list")
                .show(ui, |ui| {
                    self.render_modlist(ui);
                });
        });
        self.render_log(ctx);
    }
}

pub fn main() {
    crate::logger::init();
    log::debug!("Logger initialized");
    log::info!("Started ukmm");
    eframe::run_native(
        "U-King Mod Manager",
        NativeOptions::default(),
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

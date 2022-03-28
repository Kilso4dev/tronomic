use super::app_graph;
use crate::dmx;
use epi::App;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
enum ScreenState {
    Fixtures,
    Output,
    Nodetree,
    Sequencer,
    Live,
    Plain,
}

#[derive(Debug, Clone)]
pub struct GuiApp {
    tron_state: crate::TronomicState,
    screen_state: ScreenState,
    counter: i32,
    //graph_ctx: egui_node_graph::Context,
}

impl App for GuiApp {
    fn name(&self) -> &str {
        "Egui test app"
    }

    fn setup(
        &mut self,
        ctx: &egui::Context,
        _frame: &epi::Frame,
        _storage: Option<&dyn epi::Storage>,
    ) {
        //ctx.set_pixels_per_point(ctx.pixels_per_point() * 2.);
        /*
        let mut fontdefs = egui::FontDefinitions::default();
        let font_dat = &mut fontdefs.font_data;
        let fams = &mut fontdefs.families;

        font_dat.insert();

        fams.insert(egui::FontFamily::Proportional, vec!["assets/fonts/Roboto/Roboto-Regular.ttf".to_string()]);
        fams.insert(egui::FontFamily::Name("Bold".into()), vec!["assets/fonts/Roboto/Roboto-Bold.ttf".to_string()]);

        let fonts = ctx.set_fonts(fontdefs);
        let style = (*ctx.style()).clone();
        */
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {

        egui::TopBottomPanel::top("main_menu_bar").show(ctx, |ui| {
            egui::trace!(ui);
            self.menu_bar_content(ui);
        });
        egui::TopBottomPanel::bottom("screen_bar").show(ctx, |ui| {
            egui::trace!(ui);
            self.bottom_bar_content(ui);
        });
        egui::CentralPanel::default().show(ctx, |ui| match self.screen_state {
            ScreenState::Fixtures => {
                ui.label("Fixtures");
            }
            ScreenState::Nodetree => {
                app_graph::node_graph(&mut self.tron_state.graph.write(), ui);
            }
            ScreenState::Output => {
                ui.label("Output configuration");
            }
            ScreenState::Sequencer => {
                ui.label("Sequencer");
            }
            ScreenState::Live => {
                ui.label("Live environment");
            }
            ScreenState::Plain => {
                Self::sliders_ui(ui, &mut self.tron_state.dmx_state.write());
            }
        });

        egui::Window::new("DmxAppWindow")
            .resizable(true)
            //.frame(egui::Frame::dark_canvas(&ctx.style()))
            .show(ctx, |ui| Self::ui_counter(ui, &mut self.counter));
    }
}

impl GuiApp {
    pub fn new(
        tron_state: super::TronomicState,
    ) -> Self {
        Self {
            tron_state,
            counter: 0,
            screen_state: ScreenState::Fixtures,
        }
    }

    fn menu_bar_content(&self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.menu_button("File", |ui| {
                if ui.button("Open").clicked() {
                    println!("File Open");
                };
                if ui.button("Save").clicked() {
                    println!("File Save");
                }
            });

            ui.menu_button("Edit", |ui| {
                if ui.button("Copy").clicked() {
                    println!("Edit Copy")
                }
                if ui.button("Paste").clicked() {
                    println!("Edit Paste")
                }
            });
            ui.menu_button("View", |ui| {
                if ui.button("A panel").clicked() {
                    println!("A panel")
                }
                if ui.button("B panel").clicked() {
                    println!("B panel")
                }
            });
        });
    }
    fn bottom_bar_content(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            ui.with_layout(egui::Layout::left_to_right(), |ui| {
                if ui.button("Fixtures").clicked() {
                    self.screen_state = ScreenState::Fixtures;
                }
                if ui.button("Output Config").clicked() {
                    self.screen_state = ScreenState::Output;
                }
                if ui.button("Plain Sliders").clicked() {
                    self.screen_state = ScreenState::Plain;
                }
                if ui.button("Node tree").clicked() {
                    self.screen_state = ScreenState::Nodetree;
                }
                if ui.button("Sequencer").clicked() {
                    self.screen_state = ScreenState::Sequencer;
                }
                if ui.button("Live environment").clicked() {
                    self.screen_state = ScreenState::Live;
                }
            });

            ui.with_layout(egui::Layout::right_to_left(), |ui| {
                egui::widgets::global_dark_light_mode_switch(ui);
                ui.separator();
                egui::warn_if_debug_build(ui);
                ui.spacing();
                ui.label(format!("Eval {:4.3}", *self.tron_state.fps_eval.read()));
                ui.spacing();
                ui.label(format!("Out {:4.3}", *self.tron_state.fps_outp.read()));
                ui.spacing();
            });
        });
    }

    fn sliders_ui(
        ui: &mut egui::Ui,
        dmx_state: &mut parking_lot::RwLockWriteGuard<'_, dmx::DmxState>,
    ) {
        egui::ScrollArea::both()
            .auto_shrink([true; 2])
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    for (un_id, un) in dmx_state.universes.iter_mut() {
                        ui.vertical(|ui| ui.label(format!("{un_id}")));
                        ui.horizontal(|ui| {
                            for i in 0..512 {
                                let c_val = un.get(i);
                                let mut new = c_val;
                                GuiApp::one_slider_ui(ui, i + 1, &mut new);
                                if c_val != new {
                                    un.set(i, new);
                                }
                            }
                        });
                    }
                });
            });
    }

    fn one_slider_ui(ui: &mut egui::Ui, i: usize, chan: &mut u8) {
        ui.group(|ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center).with_cross_justify(false).with_main_justify(false), |ui| {
                ui.add(egui::Label::new(i.to_string()).wrap(false));
                ui.add(
                    egui::Slider::new(chan, 0..=u8::MAX)
                        .show_value(false)
                        .vertical(),
                );
                ui.add(egui::DragValue::new(chan))
            });
        });
    }

    fn ui_counter(ui: &mut egui::Ui, counter: &mut i32) {
        ui.horizontal(|ui| {
            if ui.button("-").clicked() {
                *counter -= 1;
            }
            ui.label(counter.to_string());
            if ui.button("+").clicked() {
                *counter += 1;
            }
        });
    }
}

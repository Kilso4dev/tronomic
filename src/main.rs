mod color;
mod error;
mod integrations;
mod threads;
mod asset;

mod app;
mod app_graph;
mod dmx;
mod gfx;
mod graph;

use egui::FontDefinitions;
use egui_winit_platform::{Platform, PlatformDescriptor};
use env_logger;
use epi::App;
use std::time::Instant;
use std::sync::Arc;
use parking_lot::{Mutex, RwLock};
use winit::{
    event::*,
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

struct EguiState {
    platform: Platform,
    repaint_signal: std::sync::Arc<EguiRepaintSignal>,
    app: app::GuiApp,
    time: Instant,
    fps_outp: Arc<Mutex<f64>>,
    fps_eval: Arc<Mutex<f64>>,
    //demo_app: egui_demo_lib::WrapApp,
}

impl gfx::Updatable for EguiState {
    fn update(&mut self, _size: winit::dpi::PhysicalSize<u32>, scale_factor: f64) {
        self.platform.update_time(self.time.elapsed().as_secs_f64());
        self.platform.begin_frame();
        let app_out = epi::backend::AppOutput::default();
        let mut frame = epi::Frame::new(epi::backend::FrameData {
            info: epi::IntegrationInfo {
                name: "egui test",
                web_info: None,
                cpu_usage: None, // TODO(Paul) Add cpu usage
                native_pixels_per_point: Some(scale_factor as _),
                prefer_dark_mode: Some(true),
            },
            output: app_out,
            repaint_signal: self.repaint_signal.clone(),
        });

        self.app.update(&self.platform.context(), &mut frame);
        //self.demo_app.update(&self.platform.context(), &mut frame);
    }

    fn platform(&mut self) -> &mut Platform {
        &mut self.platform
    }
}

enum EguiEvt {
    RequestRedraw,
}
struct EguiRepaintSignal(std::sync::Mutex<winit::event_loop::EventLoopProxy<EguiEvt>>);

impl epi::backend::RepaintSignal for EguiRepaintSignal {
    fn request_repaint(&self) {
        self.0
            .lock()
            .unwrap()
            .send_event(EguiEvt::RequestRedraw)
            .ok();
    }
}

#[tokio::main]
async fn main() {
    env_logger::builder()
        .filter(Some(env!("CARGO_CRATE_NAME")), log::LevelFilter::Debug)
        .init();

    let evt_loop = EventLoop::with_user_event();
    let egui_repaint_sig = std::sync::Arc::new(EguiRepaintSignal(std::sync::Mutex::new(
        evt_loop.create_proxy(),
    )));
    let window = WindowBuilder::new()
        .with_decorations(true)
        .with_resizable(true)
        .with_transparent(true)
        .with_title(format!(
            "{} - {}",
            env!("CARGO_PKG_NAME"),
            env!("CARGO_PKG_VERSION")
        ))
        .build(&evt_loop)
        .unwrap();

    let egui_statebuilder_repaint_sig = egui_repaint_sig.clone();
    let egui_state = {
        let size = window.inner_size();

        let style = egui::Style {
            debug: egui::style::DebugOptions {
                debug_on_hover: false,
                show_expand_width: false,
                show_expand_height: false,
                show_resize: false,
            },
            ..egui::Style::default()
        };

        let dmx = Arc::new(RwLock::new(dmx::DmxState::new(3)));
        let node_graph = Arc::new(RwLock::new(egui_node_graph::GraphEditorState::new(1., app_graph::MyGraphState::default())));
        let fps_eval = Arc::new(Mutex::new(0.));
        let fps_outp = Arc::new(Mutex::new(0.));

        EguiState {
            time: Instant::now(),
            repaint_signal: egui_statebuilder_repaint_sig,
            app: app::GuiApp::new(dmx, fps_eval.clone(), fps_outp.clone(), node_graph),
            //demo_app: egui_demo_lib::WrapApp::default(),
            platform: Platform::new(PlatformDescriptor {
                font_definitions: FontDefinitions::default(),
                physical_width: size.width,
                physical_height: size.height ,
                scale_factor: window.scale_factor(),
                style,
            }),
            fps_outp,
            fps_eval,
        }
    };

    let eval_mon = egui_state.fps_eval.clone();
    let output_mon = egui_state.fps_outp.clone();

    tokio::spawn(threads::output::output_send(50., output_mon));
    tokio::spawn(threads::evaluation::process_eval(50., eval_mon));

    let mut state = gfx::State::new(&window, egui_state).await;

    evt_loop.run(move |evt, _, ctrl_flow| {
        state.egui_state.platform.handle_event(&evt); // TODO(Paul) better way to contain into State?

        match evt {
            Event::WindowEvent {
                event: ref evt,
                window_id,
            } if window_id == window.id() => match evt {
                WindowEvent::CloseRequested
                /*| WindowEvent::KeyboardInput {
                    input:
                        KeyboardInput {
                            state: ElementState::Pressed,
                            virtual_keycode: Some(VirtualKeyCode::Escape),
                            ..
                        },
                    ..
                }*/ => *ctrl_flow = ControlFlow::Exit,
                WindowEvent::ScaleFactorChanged {
                    scale_factor,
                    new_inner_size,
                } => {
                    // TODO(Ersetzen durch besseres Scaling)
                    state.resize(**new_inner_size, Some(*scale_factor));
                }
                WindowEvent::Resized(phys_size) => state.resize(*phys_size, None),
                _ => (),
            },
            Event::RedrawRequested(win_id) if win_id == window.id() => {
                match state.render(&window) {
                    Ok(()) => (),
                    Err(wgpu::SurfaceError::Lost) => state.resize(state.size, None),
                    Err(wgpu::SurfaceError::OutOfMemory) => *ctrl_flow = ControlFlow::Exit,
                    Err(e) => log::error!("{:?}", e),
                }
            }
            Event::MainEventsCleared | Event::UserEvent(EguiEvt::RequestRedraw) => {
                window.request_redraw();
            }
            _ => (),
        }
    });
}

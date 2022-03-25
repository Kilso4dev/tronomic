use winit::window::Window;

use egui_wgpu_backend::{RenderPass, ScreenDescriptor};
use egui_winit_platform::Platform;

pub trait Updatable {
    fn update(&mut self, size: winit::dpi::PhysicalSize<u32>, scale_factor: f64);
    fn platform(&mut self) -> &mut Platform;
}


pub struct State<S: Updatable> {
    surf: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    cfg: wgpu::SurfaceConfiguration,

    pub size: winit::dpi::PhysicalSize<u32>,
    pub scale_factor: f64,

    pub egui_state: S,
    egui_rpass: RenderPass,
}

impl<S: Updatable> State<S> {
    pub async fn new(
        win: &Window,
        egui_state: S,
    ) -> Self {
        let size = win.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surf = unsafe { instance.create_surface(win) };
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: Some(&surf),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                    label: None,
                },
                None, // Trace path
            )
            .await
            .unwrap();
        let surf_format = surf.get_preferred_format(&adapter).unwrap();
        let cfg = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surf_format.clone(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Fifo,
        };

        let egui_rpass = RenderPass::new(&device, surf_format, 1);

        surf.configure(&device, &cfg);
        Self {
            surf,
            device,
            queue,
            cfg,
            size,
            scale_factor: win.scale_factor(),
            egui_state,
            egui_rpass,
        }
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>, new_scale_factor: Option<f64>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            if let Some(new) = new_scale_factor {
                self.scale_factor = new;
            }
            self.cfg.width = new_size.width;
            self.cfg.height = new_size.height;
            self.surf.configure(&self.device, &self.cfg);
        }
    }

    pub fn render(&mut self, win: &Window) -> Result<(), wgpu::SurfaceError> {

        let out = self.surf.get_current_texture()?;
        let view = out
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render encoder"),
            });
        {
            let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Main render pass"),
                color_attachments: &[wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.05,
                            g: 0.05,
                            b: 0.2,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });
        }

        // Egui
        self.egui_state.update(self.size, self.scale_factor);
        // (_out, paint_cmds)
        let egui::FullOutput {
            platform_output: _,
            needs_repaint: _,
            textures_delta,
            shapes,
        } = self.egui_state.platform().end_frame(Some(&win));
        let paint_jobs = self.egui_state.platform().context().tessellate(shapes);

        let screen_descr = ScreenDescriptor {
            physical_width: self.size.width,
            physical_height: self.size.height,
            scale_factor: win.scale_factor() as f32,
        };

        self.egui_rpass.add_textures(
            &self.device,
            &self.queue,
            &textures_delta,
        ).unwrap();
        /* NECESSARY? TODO
        self.egui_rpass
            .update_user_textures(&self.device, &self.queue);
        */
        self.egui_rpass
            .update_buffers(&self.device, &self.queue, &paint_jobs, &screen_descr);
        self.egui_rpass
            .execute(&mut encoder, &view, &paint_jobs, &screen_descr, None)
            .unwrap(); // TODO(Paul) remove unwrap in favor of another error catching method
        // Egui end
        self.queue.submit(std::iter::once(encoder.finish()));
        out.present();

        Ok(())
    }
}

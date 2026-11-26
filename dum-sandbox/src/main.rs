use plam::{Mat4, Vec3};

const EYE: Vec3 = Vec3::unchecked_new(0.0, 0.0, 10.0);
const TARGET: Vec3 = Vec3::ZERO;
const UP: Vec3 = Vec3::UP;

fn main() -> Result<(), glumy::EventLoopError> {
    DumApp::default().run()
}

#[derive(Default)]
struct DumApp {
    orchest: Option<dum::RawOrchestrator>,
    camera: Option<dum::Camera>,
    gpu_camera: Option<dum::RawGpuCamera>,
}

impl DumApp {
    pub fn run(self) -> Result<(), glumy::EventLoopError> {
        glumy::WindowApplication::new(glumy::WindowAttributes::default(), self).run()
    }
}

impl glumy::Application for DumApp {
    fn start(&mut self, window: &impl glumy::WindowFrontend, handle: &impl glumy::WindowHandle) {
        let target = handle.target().unwrap();

        let mut orchest =
            dum::RawOrchestrator::block_on_new(target.raw_window(), target.raw_display()).unwrap();

        orchest.resize(window.size());

        let mut camera = dum::Camera::default();

        camera.set_aspect_from_surface(window.size());

        let view = Mat4::unchecked_look_at_rh(EYE, TARGET, UP);

        self.gpu_camera = Some(orchest.create_gpu_camera(camera.view_projection_matrix(view)));
        self.camera = Some(camera);

        self.orchest = Some(orchest);
    }

    fn event(&mut self, event: glumy::WindowEvent, _: glumy::WindowId) {
        let Some(orchest) = &mut self.orchest else {
            return;
        };

        match event {
            glumy::WindowEvent::SurfaceResized(extent) => {
                orchest.resize(extent);

                if let Some(camera) = &mut self.camera {
                    camera.set_aspect_from_surface(extent);
                }
            }
            glumy::WindowEvent::RedrawRequested => {
                if let Ok(mut frame) = orchest.begin_frame() {
                    if let (Some(gpu_camera), Some(camera)) = (&self.gpu_camera, &self.camera) {
                        let view = Mat4::unchecked_look_at_rh(EYE, TARGET, UP);

                        gpu_camera.update(orchest.device(), camera.view_projection_matrix(view));
                    }

                    frame.render_pass(
                        dum::RenderPassDescriptor {
                            clear_color: dum::Color::BLACK,
                        },
                        |pass| {
                            if let Some(gpu_camera) = &self.gpu_camera {
                                pass.set_camera(gpu_camera);
                            }

                            pass.draw(
                                &orchest.create_gpu_mesh(&dum::Mesh::triangle_2d()),
                                &orchest.create_gpu_model(dum::Transform::default().matrix()),
                                &orchest.create_gpu_material(&dum::Material::new(dum::Color::RED)),
                            );
                        },
                    );

                    orchest.end_frame(frame);
                }
            }
            _ => {}
        }
    }
}

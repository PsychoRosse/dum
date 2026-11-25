fn main() -> Result<(), glumy::EventLoopError> {
    DumApp::default().run()
}

#[derive(Default)]
struct DumApp {
    orchest: Option<dum::RawOrchestrator>,
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

        self.orchest = Some(orchest);
    }

    fn event(&mut self, event: glumy::WindowEvent, _: glumy::WindowId) {
        let Some(orchest) = &mut self.orchest else {
            return;
        };

        match event {
            glumy::WindowEvent::RedrawRequested => match orchest.begin_frame() {
                Ok(mut frame) => {
                    frame.render_pass(
                        dum::RenderPassDescriptor {
                            clear_color: dum::Color::BLACK,
                        },
                        |pass| {
                            let mesh = &orchest.create_gpu_mesh(&dum::Mesh::triangle_2d());
                            let model = &orchest.create_gpu_model(dum::Model::new().matrix());
                            let material = &orchest
                                .create_gpu_material(&dum::Material::from_color(dum::Color::RED));

                            pass.draw(mesh, model, material);
                        },
                    );

                    orchest.end_frame(frame);
                }
                _ => {}
            },
            _ => {}
        }
    }
}

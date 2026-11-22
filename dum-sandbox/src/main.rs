use dum::{Color, FrameError, Mesh, RenderPassDescriptor};

fn main() -> Result<(), glumy::EventLoopError> {
    DumApp::default().run()
}

#[derive(Default)]
struct DumApp {
    orchest: Option<dum::Orchestrator>,
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
            dum::Orchestrator::block_on_new(target.raw_window(), target.raw_display()).unwrap();

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
                        RenderPassDescriptor {
                            clear_color: Color::BLACK,
                        },
                        |pass| {
                            pass.draw(&orchest.create_gpu_mesh(&Mesh::cube()));
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

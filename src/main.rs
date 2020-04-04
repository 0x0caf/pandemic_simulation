use chrono::Local;
use rgx::color::Rgba;
use rgx::core::*;
use rgx::kit;

use winit::{
    event::Event,
    event_loop::{ControlFlow, EventLoop},
    window::Window,
};

mod area;
mod grid_system;
mod organism;
mod simulation_app;
mod square;
mod window_box;
use simulation_app::{SimulationApp, WindowAttributes};

fn main() -> Result<(), std::io::Error> {
    let event_loop = EventLoop::new();
    let window = Window::new(&event_loop).unwrap();
    window.set_resizable(false);
    window.set_title("Pandemic Simulation");

    let mut r = Renderer::new(&window)?;
    let win = window.inner_size();
    let pip: kit::shape2d::Pipeline = r.pipeline(Blending::default());
    let mut chain = r.swap_chain(win.width as u32, win.height as u32, PresentMode::default());

    let mut simulation = SimulationApp::new(WindowAttributes {
        width: win.width as i32,
        height: win.height as i32,
    });

    let mut last_time = Local::now().timestamp_millis();

    event_loop.run(move |event, _, control_flow| match event {
        Event::WindowEvent { event, .. } => match event {
            _ => *control_flow = ControlFlow::Poll,
        },
        Event::MainEventsCleared => {
            *control_flow = ControlFlow::Wait;

            // get delta millis
            let current_time = Local::now().timestamp_millis();
            let delta_time = current_time - last_time;
            last_time = current_time;

            simulation.update(delta_time);

            let app_batch = simulation.render();
            let buffer = app_batch.finish(&r);

            let out = chain.next();
            let mut frame = r.frame();

            r.update_pipeline(
                &pip,
                kit::ortho(out.width, out.height, Default::default()),
                &mut frame,
            );

            {
                let pass = &mut frame.pass(PassOp::Clear(Rgba::TRANSPARENT), &out);
                pass.set_pipeline(&pip);
                pass.draw_buffer(&buffer);
            }
            r.present(frame);

            *control_flow = ControlFlow::Poll;
            window.request_redraw();
        }
        _ => *control_flow = ControlFlow::Poll,
    });
}

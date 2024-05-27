extern crate glium;

use glium::{glutin,Surface};

fn main() {
    let event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb,cb,&event_loop).unwrap();

    event_loop.run(move |event, _, control_flow| {
        match event {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
            }
            _ => (),
        },
        glutin::event::Event::NewEvents(cause) => match cause {
            glutin::event::StartCause::Init => {
                let mut target = display.draw();
                target.clear_color(0.0, 0.0, 1.0, 1.0);
                target.finish().unwrap();
            }
            _ => (),
        },
        _ => (),
        }
    });
}
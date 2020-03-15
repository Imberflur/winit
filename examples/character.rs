use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("Testing character events!")
        .with_inner_size(winit::dpi::LogicalSize::new(256.0, 256.0))
        .build(&event_loop)
        .unwrap();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::ReceivedCharacter(c),
                ..
            } => println!("character: {:?}", c),
            Event::MainEventsCleared => {
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
            _ => (),
        }
    });
}

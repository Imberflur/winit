use simple_logger::SimpleLogger;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

fn main() {
    SimpleLogger::new().init().unwrap();
    let event_loop = EventLoop::new();

    let window = WindowBuilder::new()
        .with_title("A fantastic window!")
        .with_inner_size(winit::dpi::LogicalSize::new(128.0, 128.0))
        .build(&event_loop)
        .unwrap();

    let mut poll_count = 0;
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;
        //println!("{:?}", event);

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                window_id,
            } if window_id == window.id() => *control_flow = ControlFlow::Exit,
            Event::WindowEvent {
                event: WindowEvent::KeyboardInput { input, .. },
                ..
            } => {
                println!("{:3} win {:?}", poll_count, input);
            }
            Event::DeviceEvent {
                event: winit::event::DeviceEvent::Key(input),
                ..
            } => {
                println!("{:3} dev: {:?}", poll_count, input);
            }
            Event::MainEventsCleared => {
                poll_count = (poll_count + 1) % 1000;
                window.request_redraw();
                // 5 polls per sec
                std::thread::sleep(std::time::Duration::from_millis(200));
            }
            _ => (),
        }
    });
}

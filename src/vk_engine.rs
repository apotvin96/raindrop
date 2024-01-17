use ash::vk::Extent2D;
use winit::{
    event::{ElementState, Event, KeyEvent, WindowEvent},
    event_loop::EventLoop,
    keyboard::{Key, NamedKey},
    window::{Window, WindowBuilder},
};

pub struct VulkanEngine {
    is_initialized: bool,
    frame_number: u32,
    stop_rendering: bool,
    window_extent: Extent2D,
    window: Option<winit::window::Window>,
    event_loop: Option<EventLoop<()>>,
}

impl VulkanEngine {
    pub fn new() -> VulkanEngine {
        VulkanEngine {
            is_initialized: false,
            frame_number: 0,
            stop_rendering: false,
            window_extent: Extent2D {
                width: 800,
                height: 600,
            },
            window: None,
            event_loop: None,
        }
    }

    pub fn init(&mut self) {
        println!("Initializing Vulkan Engine");

        let event_loop = EventLoop::new().unwrap();
        let window = Window::new(&event_loop).unwrap();

        window.set_title("Vulkan");

        self.event_loop = Some(event_loop);
        self.window = Some(window);

        self.is_initialized = true;
    }

    pub fn run(&mut self) {
        println!("Running Vulkan Engine");

        let window = self.window.as_ref().unwrap();

        // Event loop, I believe since it is a blocking setup, requires taking ownership of itself so I have to take here
        //     Thank you designers, this makes no sense to me but I'm probably just dumb
        self.event_loop
            .take()
            .unwrap()
            .run(move |event, window_target| match event {
                Event::WindowEvent { event, window_id } if window_id == window.id() => {
                    match event {
                        WindowEvent::CloseRequested => window_target.exit(),
                        WindowEvent::RedrawRequested => {
                            window.pre_present_notify();
                        }
                        WindowEvent::KeyboardInput {
                            event:
                                KeyEvent {
                                    logical_key: key,
                                    state: ElementState::Pressed,
                                    ..
                                },
                            ..
                        } => match key.as_ref() {
                            Key::Named(NamedKey::Escape) => {
                                window_target.exit();
                            }
                            _ => ()
                        },
                        _ => (),
                    }
                }
                _ => (),
            })
            .expect("Failed to run event loop");
    }

    pub fn cleanup(&self) {
        println!("Cleaning up Vulkan Engine");
    }
}

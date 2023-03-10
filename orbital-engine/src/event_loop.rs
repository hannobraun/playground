use winit::{
    event::{Event, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

use crate::{host::Host, renderer::Renderer, watcher::Watcher};

pub async fn run() -> anyhow::Result<()> {
    let event_loop = EventLoop::new();
    let _watcher = Watcher::new(event_loop.create_proxy())?;
    let mut handler = EventLoopHandler::new(&event_loop).await?;

    event_loop.run(move |event, _, control_flow| {
        let exit = handler.handle_event(event).unwrap();
        if exit {
            *control_flow = ControlFlow::Exit;
        }
    })
}

struct EventLoopHandler {
    window: Window,
    renderer: Renderer,
    host: Host,
    color: [f64; 4],
}

impl EventLoopHandler {
    async fn new(event_loop: &EventLoop<()>) -> anyhow::Result<Self> {
        let window = WindowBuilder::new()
            .with_maximized(true)
            .build(event_loop)?;
        let renderer = Renderer::new(&window).await?;
        let host = Host::new().await?;

        let color = Default::default();

        Ok(Self {
            window,
            renderer,
            host,
            color,
        })
    }

    fn handle_event(&mut self, event: Event<()>) -> anyhow::Result<bool> {
        match event {
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(size) => {
                    self.renderer.update_surface_size(size.width, size.height)
                }
                WindowEvent::CloseRequested => {
                    return Ok(true);
                }
                WindowEvent::KeyboardInput { input, .. } => {
                    if let Some(VirtualKeyCode::Escape) = input.virtual_keycode
                    {
                        return Ok(true);
                    }
                }
                _ => {}
            },
            Event::UserEvent(()) => {
                println!("Game code updated.");
            }
            Event::MainEventsCleared => {
                self.color = self.host.color()?;
                self.window.request_redraw();
            }
            Event::RedrawRequested(_) => {
                self.renderer.draw(self.color)?;
            }
            _ => {}
        }

        Ok(false)
    }
}

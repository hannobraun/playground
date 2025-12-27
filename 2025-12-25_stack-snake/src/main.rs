use std::{fs::File, io::Read, panic, path::Path, sync::Arc, thread};

use crossbeam_channel::unbounded;
use notify::{RecursiveMode, Watcher};
use pixels::{Pixels, SurfaceTexture};
use stack_assembly::Eval;
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, EventLoop},
    window::{Window, WindowAttributes, WindowId},
};

fn main() -> anyhow::Result<()> {
    let handle = thread::spawn(run_script);

    let event_loop = EventLoop::new()?;

    let mut app = WindowApp {
        window: None,
        pixels: None,
    };
    event_loop.run_app(&mut app)?;

    match handle.join() {
        Ok(result) => result?,
        Err(err) => {
            panic::resume_unwind(err);
        }
    }

    Ok(())
}

fn run_script() -> anyhow::Result<()> {
    let (notify_tx, notify_rx) = unbounded();

    let mut watcher = notify::recommended_watcher(notify_tx)?;
    watcher.watch(Path::new("snake.stack"), RecursiveMode::NonRecursive)?;

    let mut run = 0;

    'outer: loop {
        let mut script = String::new();
        File::open("snake.stack")?.read_to_string(&mut script)?;

        let mut eval = Eval::start(&script);

        let effect = eval.run();
        eprintln!("{run}: Script triggered effect: {effect:?}");

        'inner: loop {
            let event = notify_rx.recv()??;

            match event.kind {
                notify::EventKind::Modify(_) => {
                    run += 1;
                    continue 'outer;
                }
                _ => {
                    continue 'inner;
                }
            }
        }
    }
}

struct WindowApp {
    window: Option<Arc<Window>>,
    pixels: Option<Pixels<'static>>,
}

impl WindowApp {
    pub fn init(&mut self, event_loop: &ActiveEventLoop) -> anyhow::Result<()> {
        let window = {
            let window = event_loop.create_window(
                WindowAttributes::default().with_title("Snake | StackAssembly"),
            )?;

            Arc::new(window)
        };

        let pixels = {
            let size = window.inner_size();
            let surface_texture =
                SurfaceTexture::new(size.width, size.height, window.clone());
            Pixels::new(size.width, size.height, surface_texture)?
        };

        self.window = Some(window);
        self.pixels = Some(pixels);

        Ok(())
    }
}

impl ApplicationHandler for WindowApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if let Err(err) = self.init(event_loop) {
            eprintln!("Error creating window: {err:?}");
            event_loop.exit();
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _: WindowId,
        event: WindowEvent,
    ) {
        let Some(pixels) = &self.pixels else {
            return;
        };

        if let WindowEvent::RedrawRequested = event
            && let Err(err) = pixels.render()
        {
            eprintln!("Failed to draw pixels: {err:?}");
            event_loop.exit();
        }
    }
}

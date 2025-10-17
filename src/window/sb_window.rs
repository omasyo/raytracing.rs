use std::num::NonZeroU32;
use std::rc::Rc;
use softbuffer::Surface;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::error::EventLoopError;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};
use crate::window::{close, init, redraw, resize, run, RawSurface, RawWindow, WindowProperties};

/// Wrapper for Softbuffer and a Winit window.
pub struct SoftbufferWindow {
    window: RawWindow,
    surface: RawSurface,
    loop_fn: Option<Box<dyn FnMut(&mut SoftbufferWindow, WindowEvent)>>,
    properties: WindowProperties,
}

impl ApplicationHandler for SoftbufferWindow {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        (self.window, self.surface) = init(event_loop, &self.properties);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        // Automatic event handling
        close(event_loop, &event);
        resize(&event, &mut self.surface);


        let is_redraw_requested = event == WindowEvent::RedrawRequested;
        // User event handling
        if self.loop_fn.is_some() {
            let mut loop_fn = self.loop_fn.take().unwrap();
            loop_fn(self, event);
            self.loop_fn = Some(loop_fn);
        }

        // Displays buffer automatically if event is RedrawRequested
        if is_redraw_requested {
            redraw(&mut self.window, &mut self.surface);
        }
    }
}

impl SoftbufferWindow {
    /// Creates a new SoftbufferWindow.
    /// Example usage:
    /// ```rust
    /// use softbuffer_quickstart::{SoftbufferWindow, WindowProperties};
    /// let window = SoftbufferWindow::new(WindowProperties::default());
    /// ```
    pub fn new(properties: WindowProperties) -> Self {
        SoftbufferWindow {
            window: None,
            loop_fn: None,
            surface: None,
            properties,
        }
    }

    /// Runs a SoftbufferWindow event loop.
    /// To handle events, you need winit's `WindowEvent` enum.
    /// Example usage:
    /// ```rust
    /// use winit::event::WindowEvent;
    /// use softbuffer_quickstart::{SoftbufferWindow, WindowProperties};
    /// let mut window = SoftbufferWindow::new(WindowProperties::default());
    /// window.run(move |window, event| {
    ///     match event {
    ///         WindowEvent::RedrawRequested => (),
    ///         _ => ()
    ///     }
    /// })?;
    /// ```
    pub fn run(
        &mut self,
        event_fn: impl FnMut(&mut SoftbufferWindow, WindowEvent) + 'static,
    ) -> Result<(), EventLoopError> {
        self.loop_fn = Some(Box::new(event_fn));
        run(self)
    }

    /// Returns the size of a window as a tuple
    pub fn inner_size(&mut self) -> (usize, usize) {
        let size = self.window.clone().unwrap().inner_size();
        (size.width as usize, size.height as usize)
    }

    /// Gets a mutable reference to the buffer
    pub fn buffer_mut(&mut self) -> softbuffer::Buffer<'_, Rc<Window>, Rc<Window>> {
        self.surface.as_mut().unwrap().buffer_mut().unwrap()
    }

    /// Gets a mutable reference to the window
    pub fn window_mut(&mut self) -> Rc<Window> {
        self.window.clone().unwrap()
    }
}

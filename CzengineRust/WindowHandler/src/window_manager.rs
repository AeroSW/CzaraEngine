use std::{borrow::BorrowMut, cell::RefCell, ops::Deref, rc::Rc};
use winit::{
    application::ApplicationHandler,
    dpi::LogicalSize,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop}, 
    window::{self, Window, WindowAttributes}
};

pub struct WindowManager {
    pub window: RefCell<Option<Rc<Window>>>,
    pub window_attr: RefCell<WindowAttributes>
}

impl ApplicationHandler for WindowManager {
    
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = (Deref::deref(&self.window_attr.borrow()).clone());
        let mutBorrow = self.window.borrow_mut().clone();
        if let None = mutBorrow {
            let window_ref = Rc::new(event_loop.create_window(window_attributes.clone()).unwrap());
            self.window.replace(Some(window_ref.clone()));
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                // Redraw the application.
                //
                // It's preferable for applications that do not render continuously to render in
                // this event rather than in AboutToWait, since rendering in here allows
                // the program to gracefully handle redraws requested by the OS.

                // Draw.

                // Queue a RedrawRequested event.
                //
                // You only need to call this if you've determined that you need to redraw in
                // applications which do not always need to. Applications that redraw continuously
                // can render here instead.
                if let Some(ref mut rcWindow) = *self.window.borrow_mut() {
                    (*rcWindow).request_redraw();
                }
            }
            _ => (),
        }
    }
}

pub fn renderLoop() -> impl FnOnce() -> Rc<WindowManager> {
    let event_loop = EventLoop::new().unwrap();
    let mut window_manager = WindowManager{ 
        window: RefCell::new(None),
        window_attr: RefCell::new(Window::default_attributes()
            .with_title("Starting App")
            .with_inner_size(LogicalSize::new(
                1080,
                720
            ))
        )
    };
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.run_app(&mut window_manager);
    let ii = Rc::new(window_manager);
    return (|wm| (|| wm))(ii);
}

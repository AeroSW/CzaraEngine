use std::{cell::RefCell, ops::{Deref, DerefMut}, rc::Rc};

use winit::window::Window;
use WindowHandler::{WindowManager, renderLoop};
use RenderHandler::{RenderEngine, VulkanEngine};

pub struct Dependencies {
    windowManager: Rc<WindowManager>,
    renderEngine: Rc<dyn RenderEngine>
}
pub unsafe fn injectDependencies() -> Dependencies {
    let window_functor = renderLoop();
    let wr = window_functor();
    let w = &wr.window;
    let ww = w.borrow();
    let www = <Option<Rc<Window>> as Clone>::clone(&ww).unwrap();
    let wwww = Rc::<Window>::try_unwrap(www).unwrap();
    Dependencies {
        windowManager: wr.clone(),
        renderEngine: Rc::new(VulkanEngine::new(Rc::new(RefCell::new(wwww))))
    }
}
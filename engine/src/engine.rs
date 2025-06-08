use std::{mem, os::raw::c_void};

use sdl2::{
    EventPump,
    event::{self, Event},
    keyboard::Scancode,
    video::{GLContext, Window},
};

use crate::{camera::Camera, element::Element};

pub struct Engine {
    window: Window,
    _gl_context: GLContext,
    event_pump: EventPump,
    pub camera: Camera,
    elements: Vec<Element>,
    hooks: Vec<Box<dyn FnMut(&mut Engine)>>,
}
impl Engine {
    pub fn new(title: &str, camera: Camera) -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let video = sdl.video()?;

        let gl_attr = video.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);

        let window = video
            .window(title, 800, 800)
            .opengl()
            .resizable()
            .build()
            .unwrap();
        let _gl_context = window.gl_create_context().unwrap();
        let _gl = gl::load_with(|s| video.gl_get_proc_address(s) as *const c_void);

        let event_pump = sdl.event_pump()?;
        Ok(Self {
            event_pump,
            window,
            _gl_context,
            camera,
            elements: Vec::new(),
            hooks: Vec::new(),
        })
    }
    pub fn add_hook(mut self, hook: impl FnMut(&mut Engine) + 'static) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }
    pub fn events(&mut self) -> Vec<sdl2::event::Event> {
        self.event_pump.poll_iter().collect()
    }
    pub fn swap_window(&self) {
        self.window.gl_swap_window();
    }
    pub fn clear_color(&self, r: f32, g: f32, b: f32) {
        unsafe { gl::ClearColor(r, g, b, 0.0) };
    }
    fn clear(&self) {
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT) };
    }
    pub fn update_size(&self, w: i32, h: i32) {
        unsafe { gl::Viewport(0, 0, w, h) };
    }
    fn render(&self) {
        self.elements.iter().for_each(|e| e.render(&self.camera));
    }
    pub fn add_element(&mut self, element: Element) {
        self.elements.push(element);
    }
    pub fn key_pressed(&self, scan_code: Scancode) -> bool {
        self.event_pump
            .keyboard_state()
            .is_scancode_pressed(scan_code)
    }
    pub fn run(&mut self) {
        'main: loop {
            for event in self.events() {
                match event {
                    Event::Quit { .. } => break 'main,
                    Event::Window {
                        win_event: event::WindowEvent::Resized(w, h),
                        ..
                    } => {
                        self.update_size(w, h);
                    }
                    _ => {}
                }
            }

            let mut hooks = mem::take(&mut self.hooks);
            for hook in &mut hooks {
                hook(self);
            }
            self.hooks = hooks;

            self.clear();
            self.render();
            self.swap_window();
        }
    }
}

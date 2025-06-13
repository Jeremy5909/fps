use std::{mem, os::raw::c_void, time::Instant};

use physics::Physics;
use sdl2::{
    EventPump,
    event::{self, Event},
    keyboard::Scancode,
    mouse::MouseUtil,
    video::{GLContext, Window},
};

use crate::{camera::Camera, element::Element};

mod physics;

pub struct Engine<'a> {
    window: Window,
    _gl_context: GLContext,
    event_pump: EventPump,
    pub camera: Camera,
    elements: Vec<Element<'a>>,
    hooks: Vec<Box<dyn FnMut(&mut Engine)>>,
    event_hooks: Vec<Box<dyn FnMut(&mut Engine, &Event)>>,
    mouse: MouseUtil,
    last_time: Instant,
    accumulated_time: f32,
    physics: Option<Physics>,
}
impl<'a> Engine<'a> {
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
        unsafe { gl::Enable(gl::DEPTH_TEST) };
        Ok(Self {
            event_pump,
            window,
            _gl_context,
            camera,
            elements: Vec::new(),
            hooks: Vec::new(),
            event_hooks: Vec::new(),
            mouse: sdl.mouse(),
            physics: None,
            last_time: Instant::now(),
            accumulated_time: 0.0,
        })
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
        unsafe { gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT) };
    }
    pub fn update_size(&self, w: i32, h: i32) {
        unsafe { gl::Viewport(0, 0, w, h) };
    }
    fn render(&self) {
        self.elements.iter().for_each(|e| e.render(&self.camera));
    }
    pub fn key_pressed(&self, scan_code: Scancode) -> bool {
        self.event_pump
            .keyboard_state()
            .is_scancode_pressed(scan_code)
    }
    pub fn set_relative_mouse(&mut self) {
        self.mouse.set_relative_mouse_mode(true);
    }
    pub fn run(&mut self) {
        const FIXED_DT: f32 = 1.0 / 60.0;

        'main: loop {
            let now = Instant::now();
            let frame_dt = (now - self.last_time).as_secs_f32();
            self.last_time = now;

            self.accumulated_time += frame_dt;

            let mut hooks = mem::take(&mut self.hooks);
            let mut event_hooks = mem::take(&mut self.event_hooks);

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
                for hook in &mut event_hooks {
                    hook(self, &event);
                }
            }
            while self.accumulated_time >= FIXED_DT {
                if let Some(physics) = &mut self.physics {
                    physics.integreation_parameters.dt = FIXED_DT;
                }
                for hook in &mut hooks {
                    hook(self);
                }
                self.accumulated_time -= FIXED_DT;
            }

            self.hooks = hooks;
            self.event_hooks = event_hooks;

            self.clear();
            self.render();
            self.swap_window();
        }
    }
    pub fn add_element(&mut self, mut element: Element<'a>) {
        if let Some(physics) = &mut self.physics {
            if let Some(rb) = element.rigid_body.take() {
                let rb_handle = physics.rigid_body_set.insert(rb);
                if let Some(collider) = element.collider.take() {
                    let col_handle = physics.collider_set.insert_with_parent(
                        collider,
                        rb_handle,
                        &mut physics.rigid_body_set,
                    );
                    element.collider_handle = Some(col_handle);
                }
                element.rigid_body_handle = Some(rb_handle);
            }
        }
        self.elements.push(element);
    }
    pub fn add_hook(mut self, hook: impl FnMut(&mut Engine) + 'static) -> Self {
        self.hooks.push(Box::new(hook));
        self
    }
    pub fn add_event_hook(mut self, hook: impl FnMut(&mut Engine, &Event) + 'static) -> Self {
        self.event_hooks.push(Box::new(hook));
        self
    }
}

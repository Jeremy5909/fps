use sdl2::keyboard::Scancode;

use crate::engine::Engine;

pub fn wasd_flying_movement(engine: &mut Engine) {
    if engine.key_pressed(Scancode::W) {
        engine.camera.move_forward();
    }
    if engine.key_pressed(Scancode::S) {
        engine.camera.move_backward();
    }
    if engine.key_pressed(Scancode::A) {
        engine.camera.move_left();
    }
    if engine.key_pressed(Scancode::D) {
        engine.camera.move_right();
    }
    if engine.key_pressed(Scancode::Space) {
        engine.camera.move_up();
    }
    if engine.key_pressed(Scancode::LShift) {
        engine.camera.move_down();
    }
}

pub mod event_hooks {
    use sdl2::event::Event;

    use crate::engine::Engine;

    pub fn mouse_movement(engine: &mut Engine, event: &Event) {
        engine.set_relative_mouse();
        match *event {
            Event::MouseMotion { xrel, yrel, .. } => engine.camera.rotate(xrel, yrel),
            _ => {}
        }
    }
}

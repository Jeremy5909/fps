use std::f32;

use engine::{
    TextureVertex,
    camera::Camera,
    element::Element,
    engine::Engine,
    event::{self, Event},
    keyboard::Scancode
    //mouse::MouseButton,
    program::Program,
};

fn main() {
    let mut engine =
        Engine::new("fps", Camera::new(1.0, f32::consts::FRAC_PI_3, 0.1, 100.0)).unwrap();

    let mut cube = Element::new(
        vec![
            TextureVertex {
                pos: (-0.5, 0.5, 0.0).into(),
                tex_coords: (0.0, 1.0).into(),
            },
            TextureVertex {
                pos: (0.5, 0.5, 0.0).into(),
                tex_coords: (1.0, 1.0).into(),
            },
            TextureVertex {
                pos: (0.5, -0.5, 0.0).into(),
                tex_coords: (1.0, 0.0).into(),
            },
            TextureVertex {
                pos: (-0.5, -0.5, 0.0).into(),
                tex_coords: (0.0, 0.0).into(),
            },
        ],
        vec![0, 1, 2, 0, 2, 3],
        Program::from_name("shaders/textured_cube").unwrap(),
    )
    .unwrap();
    cube.add_texture("brick_wall.jpg").unwrap();
    engine.add_element(cube);
    engine.clear_color(0.7, 0.5, 1.0);
    'main: loop {
        for event in engine.events() {
            match event {
                Event::Quit { .. } => break 'main,
                Event::Window {
                    win_event: event::WindowEvent::Resized(w, h),
                    ..
                } => {
                    engine.update_size(w, h);
                }
                _ => {}
            }
        }

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

        engine.clear();
        engine.render();
        engine.swap_window();
    }
}

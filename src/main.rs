use engine::{
    TextureVertex,
    element::Element,
    engine::Engine,
    event::{self, Event},
    program::Program,
};

fn main() {
    let mut engine = Engine::new("fps").unwrap();
    let mut vertices = Vec::new();
    for z in [-0.5, 0.5] {
        for y in [-0.5, 0.5] {
            for x in [-0.5, 0.5] {
                vertices.push(TextureVertex {
                    pos: (x, y, z),
                    tex_coords: (x + 0.5, y + 0.5),
                });
            }
        }
    }

    let mut cube = Element::new(
        vertices,
        vec![
            // Back
            0, 1, 2, // Bottom
            1, 2, 3, // Top
            // Left
            0, 4, 2, // Bottom
            4, 2, 6, // Top
            // Right
            1, 5, 3, // Bottom
            5, 3, 7, // Top
            // Front
            4, 5, 7, // Bottom
            4, 6, 7, // Top
            // Top
            6, 3, 7, // Bottom
            6, 2, 3, // Top
            // Bottom
            4, 5, 1, // Bottom
            4, 0, 1, // Top
        ],
        Program::from_name("shaders/textured_square").unwrap(),
    )
    .unwrap();
    cube.add_texture("brick_wall.jpg").unwrap();

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
        engine.clear();
        cube.render();
        engine.swap_window();
    }
}

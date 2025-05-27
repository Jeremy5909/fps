use engine::{
    TextureVertex,
    element::Element,
    engine::Engine,
    event::{self, Event},
    program::Program,
};
use nalgebra::{Matrix4, Perspective3, Point3, Vector3};

fn main() {
    let mut engine = Engine::new("fps").unwrap();
    let mut vertices = Vec::new();
    for z in [-0.5, 0.5] {
        for y in [-0.5, 0.5] {
            for x in [-0.5, 0.5] {
                vertices.push(TextureVertex {
                    pos: (x, y, z).into(),
                    tex_coords: (x + 0.5, y + 0.5).into(),
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
        Program::from_name("shaders/textured_cube").unwrap(),
    )
    .unwrap();
    cube.add_texture("brick_wall.jpg").unwrap();

    // TODO: camera struct
    let aspect_ratio = 1.0;
    let fov = std::f32::consts::FRAC_PI_3; // 60 degrees
    let near = 0.1;
    let far = 100.0;

    let projection: Matrix4<f32> = Perspective3::new(aspect_ratio, fov, near, far).to_homogeneous();

    let eye = Point3::new(0.0, 0.0, 3.0); // Camera position
    let target = Point3::origin(); // What it's looking at
    let up = Vector3::y_axis(); // Up direction

    let view: Matrix4<f32> = Matrix4::look_at_rh(&eye, &target, &up);

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
        cube.render(&view, &projection);
        engine.swap_window();
    }
}

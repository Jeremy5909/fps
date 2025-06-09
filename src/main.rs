use engine::{
    element::{Element, primitives},
    engine::Engine,
    hooks,
    program::Program,
};

fn main() {
    let mut engine = Engine::new("fps", Default::default())
        .unwrap()
        .add_hook(hooks::wasd_flying_movement)
        .add_event_hook(hooks::event_hooks::mouse_movement);

    let mut cube = Element::new(
        primitives::textured_cube::verts(),
        primitives::textured_cube::indices(),
        Program::from_name("shaders/textured_cube").unwrap(),
    )
    .unwrap();
    cube.add_texture("brick_wall.jpg").unwrap();
    engine.add_element(cube);
    engine.clear_color(0.7, 0.5, 1.0);

    engine.run();
}

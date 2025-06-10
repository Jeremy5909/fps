use std::rc::Rc;

use engine::{
    Scale3, Translation3, Vector4, element::Element, engine::Engine, hooks, program::Program,
};

fn main() {
    let mut engine = Engine::new("fps", Default::default())
        .unwrap()
        .add_hook(hooks::wasd_flying_movement)
        .add_event_hook(hooks::event_hooks::mouse_movement);

    let emissive = Program::from_name("resources/shaders/emissive").unwrap();
    emissive.set_uniform_vector4("color", &Vector4::new(1.0, 1.0, 1.0, 1.0));
    let diffuse = Program::from_name("resources/shaders/diffuse").unwrap();

    let mut light = Element::from_obj("resources/models/cube.obj", "resources/textures")
        .unwrap()
        .remove(0);
    light.add_program(Rc::new(emissive)).unwrap();
    light.model = Translation3::new(1.0, 3.0, 1.0).to_homogeneous()
        * Scale3::new(0.25, 0.25, 0.25).to_homogeneous();
    engine.add_element(light);

    let mut astronaut = Element::from_obj("resources/models/astronaut.obj", "resources/textures")
        .unwrap()
        .remove(0);
    astronaut.add_program(Rc::new(diffuse)).unwrap();
    engine.add_element(astronaut);

    engine.run();
}

use engine::{element::Element, engine::Engine, hooks};

fn main() {
    let mut engine = Engine::new("fps", Default::default())
        .unwrap()
        .add_hook(hooks::wasd_flying_movement)
        .add_event_hook(hooks::event_hooks::mouse_movement);

    let mut astronaut = Element::from_obj("models/astronaut.obj").unwrap().remove(0);
    astronaut.add_program("shaders/textured").unwrap();
    engine.add_element(astronaut);

    engine.run();
}

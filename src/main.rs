use engine::{element::Element, engine::Engine, hooks};

fn main() {
    let mut engine = Engine::new("fps", Default::default())
        .unwrap()
        .add_hook(hooks::wasd_flying_movement)
        .add_event_hook(hooks::event_hooks::mouse_movement);
    engine.clear_color(0.7, 0.5, 1.0);

    let face = Element::from_obj("models/face.obj").unwrap().remove(0);
    engine.add_element(face);

    engine.run();
}

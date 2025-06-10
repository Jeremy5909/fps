use engine::{
    Point3, Translation3, Vector4, element::Element, engine::Engine, hooks, program::Program,
};

struct Light {
    pub pos: Point3<f32>,
    pub color: Vector4<f32>,
}

fn main() {
    let mut engine = Engine::new("fps", Default::default())
        .unwrap()
        .add_hook(hooks::wasd_flying_movement)
        .add_event_hook(hooks::event_hooks::mouse_movement);

    let lights = vec![
        Light {
            pos: Point3::new(2.0, 3.0, 2.0),
            color: Vector4::new(1.0, 0.0, 1.0, 1.0),
        },
        Light {
            pos: Point3::new(-2.0, 3.0, -2.0),
            color: Vector4::new(0.0, 1.0, 0.0, 1.0),
        },
    ];

    let diffuse = Program::from_name("resources/shaders/diffuse").unwrap();

    for (i, light) in lights.iter().enumerate() {
        let emissive = Program::from_name("resources/shaders/emissive").unwrap();
        emissive.set_uniform_vector4("color", &light.color);

        let mut light_element =
            Element::from_obj("resources/models/cube.obj", "resources/textures")
                .unwrap()
                .remove(0);
        light_element.add_program(emissive).unwrap();
        light_element.model = Translation3::from(light.pos).to_homogeneous();

        diffuse.set_uniform_vector3(&format!("lights[{}].position", i), &light.pos.coords);
        diffuse.set_uniform_vector4(&format!("lights[{}].color", i), &light.color);

        engine.add_element(light_element);
    }
    diffuse.set_uniform_1i("lightCount", lights.len() as i32);

    let mut astronaut = Element::from_obj("resources/models/astronaut.obj", "resources/textures")
        .unwrap()
        .remove(0);
    astronaut.add_program(diffuse).unwrap();

    engine.add_element(astronaut);

    engine.run();
}

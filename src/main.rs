use engine::{
    Point3, Scale3, Vector4, camera::Camera, element::Element, engine::Engine, hooks,
    program::Program,
};

struct Light {
    pub pos: Point3<f32>,
    pub color: Vector4<f32>,
}
impl Light {
    fn add_lights(lights: &Vec<Light>, diffuse: &mut Program) {
        for (i, light) in lights.iter().enumerate() {
            diffuse.set_uniform(&format!("lights[{}].position", i), &light.pos.coords);
            diffuse.set_uniform(&format!("lights[{}].color", i), &light.color);
        }
        diffuse.set_uniform("lightCount", &(lights.len() as i32));
    }
}

fn main() {
    let mut engine = Engine::new(
        "fps",
        Camera::default().positioned(Point3::new(0.0, 2.0, 6.0)),
    )
    .unwrap()
    .add_hook(hooks::flycam)
    .add_event_hook(hooks::event_hooks::mouse_movement);

    let lights = vec![Light {
        pos: Point3::new(0.0, 5.0, 0.0),
        color: Vector4::new(1.0, 1.0, 1.0, 1.0),
    }];

    // there should only be one diffuse program, this need to be fixed where maybe i can add
    // uniform to element and its updated on draw
    let mut diffuse = Program::from_name("resources/shaders/diffuse").unwrap();
    Light::add_lights(&lights, &mut diffuse);

    let mut astronaut = Element::from_obj("resources/models/astronaut.obj", "resources/textures")
        .unwrap()
        .remove(0);
    astronaut.add_program(&diffuse).unwrap();
    engine.add_element(astronaut);

    let mut diffuse_untextured =
        Program::from_name("resources/shaders/diffuse_untextured").unwrap();
    diffuse_untextured.set_uniform("albedo", &Vector4::new(0.7, 0.7, 0.7, 0.7));
    Light::add_lights(&lights, &mut diffuse_untextured);

    let mut ground = Element::from_obj("resources/models/plane.obj", "resources/textures")
        .unwrap()
        .remove(0);
    ground.add_program(&diffuse_untextured).unwrap();
    ground.model = Scale3::new(10.0, 10.0, 10.0).to_homogeneous();

    engine.add_element(ground);

    engine.run();
}

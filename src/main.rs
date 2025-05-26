use engine::{
    Vertex,
    element::Element,
    engine::Engine,
    event::{self, Event},
    program::Program,
};

fn main() {
    let mut engine = Engine::new("fps").unwrap();
    let square = Element::new(
        vec![
            Vertex { pos: (-0.5, -0.5) },
            Vertex { pos: (0.5, -0.5) },
            Vertex { pos: (0.5, 0.5) },
            Vertex { pos: (-0.5, 0.5) },
        ],
        vec![0, 1, 2, 2, 3, 0],
        Program::from_name("shaders/white").unwrap(),
    )
    .unwrap();

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
        square.render();
        engine.swap_window();
    }
}

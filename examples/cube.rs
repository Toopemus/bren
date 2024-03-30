use bren::renderer::{model::Model, Renderer};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut renderer = Renderer::new();
    let size = renderer.viewport_size();
    let w = size.0 as f32;
    let h = size.1 as f32;

    let mut cube =
        Model::load_from_file("examples/cube.obj").expect("File should be included and valid");
    cube.scale(20.0, 20.0, 20.0);
    cube.translate(w / 2.0, h / 2.0, 0.0);

    let mut i = 0.0;
    loop {
        i = if i < 359.0 { i + 1.0 } else { 0.0 };
        renderer.clear();
        cube.rotate(0.0, i, 0.0);
        renderer.draw_object(&cube);
        renderer.render();
        sleep(Duration::from_millis(1000 / 30));
    }
}

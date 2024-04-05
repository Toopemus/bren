use bren::renderer::{model::Model, viewport::Viewport, Renderer};

fn main() {
    let screen_size = Viewport::screen_size().unwrap();
    let viewport = Viewport::with_size_and_pos(
        screen_size.0 / 2,
        screen_size.1 / 2,
        screen_size.0 / 4,
        screen_size.1 / 4,
    );

    let mut renderer = Renderer::new(viewport);

    let mut cube =
        Model::load_from_file("examples/cube.obj").expect("File should be included and valid");

    cube.translate(0.0, 0.0, -5.0);

    renderer.clear();
    renderer.draw_object(&cube);
    renderer.render();

    loop {}
}

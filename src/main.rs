use core::time;
use std::thread::sleep;

use bren::renderer::model::Model;
// use bren::renderer::viewport::Viewport;
use bren::renderer::Renderer;

fn main() {
    // let viewport = Viewport::with_size_and_pos(92, 47, 46, 23);
    // let viewport = Viewport::with_size_and_pos(92, 47, 0, 0);
    // let mut renderer = Renderer::with_viewport(viewport);
    let mut renderer = Renderer::new();
    renderer.render();
    println!(" -+- Bren terminal drawing library -+-\n");
    println!("     choose a demo\n");
    println!("     1. Teapot from the future (3D)\n\n");
    loop {
        let mut choice = String::new();

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => three_dee(&mut renderer),
            _ => println!("Try another one"),
        }
    }
}

fn three_dee(renderer: &mut Renderer) {
    let size = renderer.viewport_size();
    let w = size.0 as f32;
    let h = size.1 as f32;
    let mut teapot = Model::load_from_file("shuttle.obj").unwrap();
    teapot.scale(20.0, 20.0, 20.0);
    teapot.translate(w / 2.0, h / 2.0, 0.0);
    renderer.clear();
    renderer.draw_object(&teapot);
    renderer.render();

    let mut i = 0.0;
    loop {
        i = if i < 359.0 { i + 1.0 } else { 0.0 };
        teapot.rotate(-70.0, i, 0.0);
        renderer.clear();
        renderer.draw_object(&teapot);
        renderer.render();
        sleep(time::Duration::from_millis(1000 / 30));
    }
}

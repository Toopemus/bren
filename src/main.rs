use core::time;
use std::thread::sleep;

use bren::Renderer;

fn main() {
    let mut renderer = Renderer::init();
    for i in 0..100 {
        renderer.clear();
        // renderer.draw_pixel(i, i);
        // renderer.draw_line(100 - i, i, 0, 0);
        renderer.draw_circle(50, 50, i);
        renderer.render();
        sleep(time::Duration::from_millis(1000));
    }
}

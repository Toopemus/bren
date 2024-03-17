use core::time;
use std::thread::sleep;

use bren::Renderer;

fn main() {
    let mut renderer = Renderer::init();
    for i in 0..100 {
        renderer.clear();
        // renderer.draw_pixel(i, i);
        renderer.draw_line(0, 0, 100 - i, i);
        renderer.render();
        sleep(time::Duration::from_millis(200));
    }
}

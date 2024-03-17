use core::time;
use std::thread::sleep;

use bren::Renderer;

fn main() {
    let mut renderer = Renderer::init();
    let size = renderer.terminal_size();
    println!("{:?}", size);
    println!("{}, {}", size.0 as i16, size.1 as i16);
    println!("{}, {}", size.0 as i16 / 2, size.1 as i16 / 2);
    let mut i = 0;
    loop {
        if i == 50 {
            i = 0
        }
        i += 1;
        renderer.clear();
        renderer.draw_pixel(0, 0);
        renderer.draw_pixel(size.0 as i16 - 1, 0);
        renderer.draw_pixel(0, size.1 as i16 - 1);
        renderer.draw_pixel(size.0 as i16 - 1, size.1 as i16 - 1);
        // renderer.draw_line(100 - i, i, 0, 0);
        renderer.draw_circle(size.0 as i16 / 2, size.1 as i16 / 2, i);
        renderer.draw_pixel(size.0 as i16 / 2, size.1 as i16 / 2);
        renderer.render();
        sleep(time::Duration::from_millis(200));
    }
}

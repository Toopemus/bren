use core::time;
use std::thread::sleep;

use bren::Renderer;
use rand::Rng;

fn main() {
    let mut renderer = Renderer::init();
    let size = renderer.terminal_size();
    let w: i16 = size.0 as i16;
    let h: i16 = size.1 as i16;
    println!("{:?}", size);
    let mut i = 0;
    loop {
        if i == 50 {
            i = 0
        }
        i += 1;
        renderer.clear();
        renderer.draw_pixel(0, 0);
        renderer.draw_pixel(w - 1, 0);
        renderer.draw_pixel(0, h - 1);
        renderer.draw_pixel(w, h - 1);
        renderer.draw_circle(w / 2, h / 2, i);
        renderer.draw_circle(w / 2, h / 2 + (i / 2), i / 3);
        renderer.draw_circle(w / 2 - (i / 3), h / 2 - (i / 3), i / 4);
        renderer.draw_circle(w / 2 + (i / 3), h / 2 - (i / 3), i / 4);

        renderer.draw_circle(w / 2 - (i / 3), h / 2 - (i / 3), i / 7);
        renderer.draw_circle(w / 2 + (i / 3), h / 2 - (i / 3), i / 7);

        renderer.draw_line(
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
        );
        renderer.draw_line(
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
        );
        renderer.draw_line(
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
        );
        renderer.draw_line(
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
        );
        renderer.draw_line(
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
            rand::thread_rng().gen_range(0..w),
            rand::thread_rng().gen_range(0..h),
        );
        renderer.render();
        sleep(time::Duration::from_millis(100));
    }
}

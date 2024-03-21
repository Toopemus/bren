use core::time;
use std::thread::sleep;

use bren::Renderer;
use rand::Rng;

fn main() {
    let mut renderer = Renderer::new();
    renderer.render();
    println!(" -+- Bren terminal drawing library -+-\n");
    println!("     choose a demo\n");
    println!("     1. Bouncy ball");
    println!("     2. Spoooky bowling ball\n\n");
    loop {
        let mut choice = String::new();

        std::io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        let choice: i32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if choice == 1 {
            bouncy_ball(&mut renderer);
        }
        if choice == 2 {
            spooky_bowling_ball(&mut renderer);
        }
    }
}

fn spooky_bowling_ball(renderer: &mut Renderer) {
    let size = renderer.terminal_size();
    let w: i16 = size.0 as i16;
    let h: i16 = size.1 as i16;
    let mut i = 0;
    loop {
        if i == 50 {
            i = 0
        }
        i += 1;
        renderer.clear();
        renderer.draw_circle(w / 2, h / 2, i);
        renderer.draw_circle(w / 2, h / 2 - (i / 2), i / 3);
        renderer.draw_circle(w / 2 - (i / 3), h / 2 + (i / 3), i / 4);
        renderer.draw_circle(w / 2 + (i / 3), h / 2 + (i / 3), i / 4);

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
        renderer.write_label(1, (h - 2) as u16, "Spooky bowling ball :O");
        renderer.render();
        sleep(time::Duration::from_millis(100));
    }
}

fn bouncy_ball(renderer: &mut Renderer) {
    let size = renderer.terminal_size();
    let w: i16 = size.0 as i16;
    let h: i16 = size.1 as i16;
    // ball properties
    let mut cx = w / 2;
    let mut cy = h / 2;
    let r = 10;
    let mut vx: i16 = 4;
    let mut vy: i16 = 5;

    loop {
        // move ball
        if cx - r <= 0 || cx + r >= w {
            vx = -1 * vx;
        }
        if cy - r <= 0 || cy + r >= h {
            vy = -1 * vy;
        }
        cx = cx + vx;
        cy = cy + vy;
        renderer.clear();
        renderer.draw_filled_circle(cx, cy, r);
        renderer.write_label(1, (h - 2) as u16, "Bouncy ball B)");
        renderer.render();
        sleep(time::Duration::from_millis(50));
    }
}

use core::time;
use std::thread::sleep;

use bren::renderer2d;
use bren::renderer3d;
use bren::renderer3d::Object;
use rand::Rng;

fn main() {
    let mut renderer = renderer2d::Renderer::new();
    let mut renderer3d = renderer3d::Renderer::new();
    renderer.render();
    println!(" -+- Bren terminal drawing library -+-\n");
    println!("     choose a demo\n");
    println!("     1. Bouncy ball");
    println!("     2. Spoooky bowling ball");
    println!("     3. Teapot from the future (3D)\n\n");
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
            1 => bouncy_ball(&mut renderer),
            2 => spooky_bowling_ball(&mut renderer),
            3 => three_dee(&mut renderer3d),
            _ => println!("Try another one"),
        }
    }
}

fn three_dee(renderer: &mut renderer3d::Renderer) {
    let size = renderer.terminal_size();
    let w = size.0 as f32;
    let h = size.1 as f32;
    let mut teapot = Object::load_from_file("teapot.obj").unwrap();
    teapot.scale(50.0, 50.0, 50.0);
    teapot.translate(w / 2.0, h / 5.0, 0.0);
    renderer.clear();
    renderer.draw_object(&teapot);
    renderer.render();

    let mut i = 0.0;
    loop {
        i = if i < 359.0 { i + 1.0 } else { 0.0 };
        teapot.rotate(10.0, i, 0.0);
        renderer.clear();
        renderer.draw_object(&teapot);
        renderer.render();
        sleep(time::Duration::from_millis(1000 / 30));
    }
}

fn spooky_bowling_ball(renderer: &mut renderer2d::Renderer) {
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

fn bouncy_ball(renderer: &mut renderer2d::Renderer) {
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

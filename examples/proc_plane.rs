use bren::renderer::{model::Model, viewport::Viewport, Renderer};
use std::{thread::sleep, time::Duration};

fn main() {
    let mut renderer = Renderer::new_wireframe(Viewport::new());

    let mut plane = Model::new_plane(16, 5.0);

    plane.translate(0.0, 0.0, -10.0);

    let mut i = 0.0;
    loop {
        for vertex in &mut plane.vertex_buffer {
            vertex.position.z = ((std::f32::consts::PI
                * (vertex.position.x + (i / std::f32::consts::PI)))
                .sin()
                / 1.5)
                * ((std::f32::consts::PI * (vertex.position.y + (i / std::f32::consts::PI))).sin()
                    / 1.5);
        }
        i += 0.1;

        plane.rotate(-20.0, 0.0, 0.0);
        renderer.clear();
        renderer.draw_object(&plane);
        renderer.render();

        sleep(Duration::from_millis(1000 / 30));
    }
}

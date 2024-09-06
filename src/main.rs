use hecs::*;
use macroquad::{input::KeyCode, prelude::*, rand};

#[derive(Debug)]
pub struct Body {
    pub position: Vec2,
    pub velocity: Vec2,
    pub size: f32,
    pub mass: f32,
    pub color: Color,
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let G: f32 = 6.67430 * f32::powf(10.0, -11.0);
    let AU: f32 = 149598000.0;
    let mut camera = Camera2D {
        rotation: 0.0,
        zoom: vec2(1.0, 1.0),
        target: vec2(0.0, 0.0),
        offset: vec2(0.0, 0.0),
        render_target: None,
        viewport: None,
    };

    let mut bodys: Vec<Body> = Vec::new();

    //for x in -20..20 {
    //    for y in -20..20 {
    //        bodys.push(Body {
    //            position: vec2(x as f32, y as f32),
    //            color: RED,
    //            mass: 1000.0,
    //            size: 1.0 / 10.0,
    //            velocity: vec2(0.0, 0.0),
    //        });
    //    }
    //}
    //

    //bodys.push(Body {
    //    position: vec2(0.0, 0.0),
    //    color: YELLOW,
    //    mass: 1000000.0,
    //    size: 0.1,
    //    velocity: vec2(0.0, 0.0),
    //});

    for _ in 0..500 {
        let p = rand::gen_range(0.001, 100.0);
        bodys.push(Body {
            position: vec2(rand::gen_range(-5.0, 5.0), rand::gen_range(-5.0, 5.0)),
            color: BLUE,
            mass: p * 100.0,
            size: p / 1000.0,
            velocity: vec2(rand::gen_range(-10.0, 10.0) / 1000.0, rand::gen_range(-5.0, 15.0) / 1000.0),
        });
    }

    println!("{:?}", bodys);

    loop {
        let delta = get_frame_time();

        if is_key_down(KeyCode::Escape) {
            break;
        }

        let (mouse_wheel_x, mouse_wheel_y) = mouse_wheel();

        let camera_new_zoom = camera.zoom.x + (mouse_wheel_y * camera.zoom.x * delta * 8.0);

        camera.zoom = vec2(camera_new_zoom, camera_new_zoom);

        //Controls
        if is_key_down(KeyCode::W) {
            camera.target.y -= 1.0 * delta / camera_new_zoom;
        }
        if is_key_down(KeyCode::S) {
            camera.target.y += 1.0 * delta / camera_new_zoom;
        }
        if is_key_down(KeyCode::A) {
            camera.target.x -= 1.0 * delta / camera_new_zoom;
        }
        if is_key_down(KeyCode::D) {
            camera.target.x += 1.0 * delta / camera_new_zoom;
        }

        set_camera(&camera);
        clear_background(BLACK);

        for i in 1..bodys.len() {
            let (left, right) = bodys.split_at_mut(i);

            let l = left.last_mut().unwrap();
            for r in right {
                //skip if far away
                //let distance = l.position.distance(vec2(0.0,0.0));
                //if distance > 0.1 {
                //    continue;
                //};
                // path from two
                let displacment_vec = l.position - r.position;
                // unit vec
                let normal_vec = displacment_vec.normalize_or_zero();
                let normal_length = displacment_vec.length_squared();

                //println!("-{} * ({}*{} / {}) * {} = {}", G, l.mass, r.mass , normal_length, normal_vec, -G * (l.mass * r.mass / normal_length) * normal_vec);

                let force = -G * (l.mass * r.mass / normal_length) * normal_vec;
                //println!("{}[{}] {}[{}] = {}", l.mass, l.position, r.mass, r.position, force);

                l.velocity += force / l.mass;
                r.velocity -= force / r.mass;
            }
        }

        for body in &mut bodys {
            body.position += body.velocity;
            draw_circle(body.position.x, body.position.y, body.size, body.color);
        }

        // UI
        set_default_camera();
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 18.0, WHITE);
        draw_text(&format!("FRAME TIME: {}", delta), 20.0, 40.0, 18.0, WHITE);
        draw_text(&format!("MOUSE: X: {} Y: {}", mouse_wheel_x, mouse_wheel_y), 20.0, 60.0, 18.0, WHITE);
        draw_text(&format!("ZOOM: {}", camera_new_zoom), 20.0, screen_height() - 20.0, 18.0, WHITE);

        next_frame().await
    }
}

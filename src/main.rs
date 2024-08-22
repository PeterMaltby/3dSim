use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut camera = Camera3D {
        position: vec3(-20.0, 15.0, 0.0),
        up: vec3(0.0, 1.0, 0.0),
        target: vec3(0.0, 0.0, 0.0),
        fovy: 45.0,
        projection: Projection::Perspective,
        render_target: None,
        aspect: None,
        viewport: None,
    };

    let mut zoom = 1.0;

    let camera_speed = 0.2;
    let camera_radius = 20.0;
    let mut camera_rotation = 0.0;

    loop {
        let delta = get_frame_time();
        // let camera_2d_radius = (f32::powi(camera.position.x - camera.target.x, 2) + f32::powi(camera.position.z - camera.target.z, 2)).sqrt();


        if is_key_down(KeyCode::Escape) {
            break;
        }

        if is_key_down(KeyCode::W) {
            camera.target.x += camera_speed;
        }
        if is_key_down(KeyCode::S) {
            camera.target.x -= camera_speed;
        }
        if is_key_down(KeyCode::A) {
            camera.target.z -= camera_speed;
        }
        if is_key_down(KeyCode::D) {
            camera.target.z += camera_speed;
        }

        //float camX = sin(glfwGetTime()) * radius;
        //float camZ = cos(glfwGetTime()) * radius;

        if is_key_down(KeyCode::Q) {
            camera_rotation += (camera_speed / 5.0) ;
        }
        if is_key_down(KeyCode::E) {
            camera_rotation -= camera_speed / 5.0;
        }

        camera.position.x = camera.target.x + (camera_rotation.sin() * camera_radius);
        camera.position.z = camera.target.z + (camera_rotation.cos() * camera_radius);

        set_camera(&camera);
        clear_background(LIGHTGRAY);

        // 3d space rendering
        draw_grid(1000, 1.0, BLACK, LIME);

        draw_line(-0.4, 0.4, -0.8, 0.9, 0.05, BLUE);
        draw_rectangle(-0.3, 0.3, 0.2, 0.2, GREEN);
        draw_circle(0., 0., 0.1, YELLOW);
        draw_sphere(camera.target, 0.1, None, RED);

        // UI
        set_default_camera();
        draw_text(&format!("FPS: {}", get_fps()), 20.0, 20.0, 18.0, DARKGRAY);
        draw_text( &format!("FRAME TIME: {}", delta), 20.0, 40.0, 18.0, DARKGRAY,);
        draw_text( &format!("camera rotation: {}", camera_rotation), 20.0, 60.0, 18.0, DARKGRAY,);
        draw_text( &format!("camera pos : {}", camera.position), 20.0, 80.0, 18.0, DARKGRAY,);
        draw_text( &format!("target pos: {}", camera.target), 20.0, 100.0, 18.0, DARKGRAY,);

        next_frame().await
    }
}

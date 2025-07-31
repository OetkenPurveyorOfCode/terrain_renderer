use macroquad::miniquad::conf::Platform;
use macroquad::miniquad::gl::{self, GL_FILL, GL_FRONT_AND_BACK, GL_LINE};
use macroquad::prelude::*;

pub mod heightmap;
pub use crate::heightmap::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dirt Jam".to_owned(),
        //fullscreen: true,
        high_dpi: true,
        sample_count: 1,
        platform: Platform {
            apple_gfx_api: miniquad::conf::AppleGfxApi::OpenGl,
            swap_interval: Some(0),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera3D {
        position: vec3(2., 2., 2.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    let camera_speed = 0.3;
    let mut stage = Heightmap::new();
    loop {
        // input
        let dt = get_frame_time();
        if is_key_down(KeyCode::E) {
            let delta = (camera.target - camera.position).normalize() * camera_speed;
            camera.position += delta*dt;
        }
        if is_key_down(KeyCode::Q) {
            let delta = (camera.target - camera.position).normalize() * camera_speed;
            camera.position -= delta*dt;
        }
        if is_key_down(KeyCode::D) {
            let rot = Quat::from_axis_angle(camera.up, dt*100.0f32.to_radians());
            let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
            camera.position = mat.transform_point3(camera.position);
            camera.up = mat.transform_vector3(camera.up);
        }
        if is_key_down(KeyCode::A) {
            let rot = Quat::from_axis_angle(camera.up, dt*-100.0f32.to_radians());
            let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
            camera.position = mat.transform_point3(camera.position);
            camera.up = mat.transform_vector3(camera.up);
        }
        if is_key_down(KeyCode::W) {
            //dbg!(camera.up, camera.target, camera.up.cross(camera.position-camera.target).normalize());
            let rot = Quat::from_axis_angle(vec3(0.0, 0.0, 1.0), dt*100.0f32.to_radians());
            let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
            camera.position = mat.transform_point3(camera.position);
            camera.up = mat.transform_vector3(camera.up);
        }
        if is_key_down(KeyCode::S) {
            let rot = Quat::from_axis_angle(vec3(0.0, 0.0, 1.0), dt*-100.0f32.to_radians());
            let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
            camera.position = mat.transform_point3(camera.position);
            camera.up = mat.transform_vector3(camera.up);
        }
        if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::W) {
            unsafe {gl::glPolygonMode(GL_FRONT_AND_BACK, GL_LINE)};
        }
        if is_key_down(KeyCode::RightShift) && is_key_down(KeyCode::W) {
            unsafe {gl::glPolygonMode(GL_FRONT_AND_BACK, GL_FILL)};
        }

        // drawing
        set_camera(&camera);
        clear_background(BLACK);
        draw_grid(20, 0.1, BLACK, GRAY);
        let model = Mat4::IDENTITY;
        stage.draw(&camera, model);
        // Back to screen space
        set_default_camera();
        draw_fps();
        //draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);

        next_frame().await
    }
}

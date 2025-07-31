use macroquad::miniquad::conf::Platform;

use macroquad::prelude::*;

pub mod camera;
pub use crate::camera::*;
pub mod heightmap;
pub use crate::heightmap::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dirt Jam".to_owned(),
        fullscreen: true,
        high_dpi: true,
        sample_count: 1,
        platform: Platform {
            apple_gfx_api: miniquad::conf::AppleGfxApi::OpenGl,
            swap_interval: Some(1),
            ..Default::default()
        },
        ..Default::default()
    }
}

#[cfg(target_family = "wasm")]
fn enable_wireframe() {}
#[cfg(target_family = "wasm")]
fn disable_wireframe() {}

#[cfg(not(target_family = "wasm"))]
fn enable_wireframe() {
    use macroquad::miniquad::gl::{self, GL_FRONT_AND_BACK, GL_LINE};
    unsafe {gl::glPolygonMode(GL_FRONT_AND_BACK, GL_LINE)};}
#[cfg(not(target_family = "wasm"))]
fn disable_wireframe() {
    use macroquad::miniquad::gl::{self, GL_FILL, GL_FRONT_AND_BACK};
    unsafe {gl::glPolygonMode(GL_FRONT_AND_BACK, GL_FILL)};}

#[macroquad::main(window_conf)]
async fn main() {
    let mut camera = Camera3D {
        position: vec3(3., 0.8, 0.0),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    use libnoise::prelude::*;
    let generator = libnoise::Source::simplex(rand::rand() as u64).fbm(5, 0.013, 2.0, 0.5);
    let mut light_dir = Vec3::new(10.0, 2.0, 0.0).normalize();
    let mut heightmap = Heightmap::new(generator, (50, 50), 45.);
    let mut fly_forward = true;
    let mut dir = 1.;
    loop {
        // input
        let dt = get_frame_time();
        if fly_forward {
            camera_move_forward(&mut camera, dt, true);
        }
        update_camera(&mut camera);
        if is_key_down(KeyCode::Escape) {
            break;
        }
        if is_key_pressed(KeyCode::T) {
            fly_forward = !fly_forward;
        }
        if is_key_down(KeyCode::LeftShift) && is_key_down(KeyCode::W) {
            enable_wireframe();
        }
        if is_key_down(KeyCode::RightShift) && is_key_down(KeyCode::W) {
            disable_wireframe();
        }
        if light_dir.y < 0. {
            dir *= -1.;
        }
        light_dir = rotate_vector_axis_angle(light_dir, vec3(0., 0., 1.), 0.3*dir*dt);

        // drawing
        set_camera(&camera);
        clear_background(BLACK);
        draw_grid(20, 0.1, BLACK, GRAY);
        heightmap.draw(&camera, light_dir);

        // Back to screen space
        set_default_camera();
        draw_fps();
        draw_text(&format!("{:?}", heightmap.chunks.len()), 10.0, 50.0, 20.0, WHITE);

        next_frame().await
    }
}

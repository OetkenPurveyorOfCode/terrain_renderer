use macroquad::miniquad::conf::Platform;
use macroquad::prelude::*;
use libnoise::prelude::*;
pub mod terrain;
pub mod heightmap;
pub use crate::terrain::*;
pub use crate::heightmap::*;

fn window_conf() -> Conf {
    Conf {
        window_title: "Dirt Jam".to_owned(),
        //fullscreen: true,
        high_dpi: true,
        sample_count: 1,
        platform: Platform{apple_gfx_api: miniquad::conf::AppleGfxApi::OpenGl, ..Default::default()},
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
    //let mesh = gen_terrain_mesh(10, 10);
    let mut stage = Heightmap::new();
    loop {
        clear_background(LIGHTGRAY);
        /*
        let mut bytes : Vec<u8> = Vec::with_capacity(4*800*600);

        let texture = Texture2D::from_rgba8(800, 600, &bytes);
        let params = DrawTextureParams {
            dest_size: Some(Vec2 {
                x: screen_width(),
                y: screen_height(),
            }),
            ..Default::default()
        };*/
        // input
        if is_key_down(KeyCode::E) {
            let delta = (camera.target-camera.position).normalize()*camera_speed;
            camera.position += delta;
        }
        if is_key_down(KeyCode::Q) {
            let delta = (camera.target-camera.position).normalize()*camera_speed;
            camera.position -= delta;
        }
        if is_key_down(KeyCode::D) {
            let rot = Quat::from_axis_angle(camera.up, 1.0f32.to_radians());
            let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
            camera.position = mat.transform_point3(camera.position);
            camera.up = mat.transform_vector3(camera.up);
        }
        if is_key_down(KeyCode::A) {
            let rot = Quat::from_axis_angle(camera.up, -1.0f32.to_radians());
            let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
            camera.position = mat.transform_point3(camera.position);
            camera.up = mat.transform_vector3(camera.up);
        }
        if is_key_down(KeyCode::W) {
            let rot = Quat::from_axis_angle(vec3(0.0, 0.0, 1.0), 1.0f32.to_radians());
            let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
            camera.position = mat.transform_point3(camera.position);
            camera.up = mat.transform_vector3(camera.up);
        }
        if is_key_down(KeyCode::S) {
            let rot = Quat::from_axis_angle(vec3(0.0, 0.0, 1.0), -1.0f32.to_radians());
            let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
            camera.position = mat.transform_point3(camera.position);
            camera.up = mat.transform_vector3(camera.up);
        }

        // drawing
        set_camera(&camera);
        //draw_mesh(&mesh);
        draw_grid(20, 0.1, BLACK, GRAY);
        stage.draw(&camera);
        draw_cube_wires(vec3(0., 0., 0.), vec3(1., 1., 1.), DARKGREEN);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);
        
        next_frame().await
    }
}


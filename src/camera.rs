use std::ffi::c_float;

use macroquad::{miniquad::gl::WGL_CONTEXT_RESET_NOTIFICATION_STRATEGY_ARB, prelude::*};

pub fn get_camera_forward(camera: &Camera3D) -> Vec3 {
    (camera.target-camera.position).normalize()
}

pub fn get_camera_up(camera: &Camera3D) -> Vec3 {
    camera.up.normalize()
}

pub fn get_camera_right(camera: &Camera3D) -> Vec3 {
    let forward = get_camera_forward(camera);
    forward.cross(get_camera_up(camera)).normalize()
}

pub fn camera_move_forward(camera: &mut Camera3D, distance: f32, move_in_world_plane: bool) {
    let mut forward = get_camera_forward(camera);
    if (move_in_world_plane) {
        forward.y = 0.;
        forward = forward.normalize();
    }
    forward = distance*forward;
    camera.position += forward;
    camera.target += forward;
}

pub fn camera_move_up(camera: &mut Camera3D, distance: f32) {
    let up = distance*get_camera_up(camera);
    camera.position += up;
    camera.target += up;
}

pub fn camera_move_right(camera: &mut Camera3D, distance: f32, move_in_world_plane: bool) {
    let mut right = get_camera_right(camera);
    if (move_in_world_plane) {
        right.y = 0.;
        right = right.normalize();
    }
    right = distance*right;
    camera.position += right;
    camera.target += right;
}

pub fn camera_move_to_target(camera: &mut Camera3D, delta: f32) {
    let mut distance = (camera.target-camera.position).length();
    distance += delta;
    distance = distance.max(0.001);
    camera.position = camera.target -distance*get_camera_forward(camera);
}

fn rotate_vector_axis_angle(input: Vec3, axis: Vec3, angle: f32) -> Vec3 {
    let rot = Quat::from_axis_angle(axis, angle);
    let mat = Mat4::from_rotation_translation(rot, vec3(0.0, 0.0, 0.0));
    return mat.transform_vector3(input);
}

pub fn camera_yaw(camera: &mut Camera3D, angle: f32, rotate_around_target: bool) {
    let up = get_camera_up(camera);
    let mut view_vector = camera.target-camera.position;
    view_vector = rotate_vector_axis_angle(view_vector, up, angle);
    if rotate_around_target {
        camera.position = camera.target - view_vector;
    }
    else {
        camera.target = camera.position + view_vector;
    }
}

pub fn camera_pitch(camera: &mut Camera3D, angle: f32, rotate_around_target: bool, lock_view: bool, rotate_up: bool) {
    let mut angle = angle;
    let up = get_camera_up(camera);
    let mut view_vector = camera.target-camera.position;
    if lock_view {
        let mut max_angle_up = (up.dot(view_vector)/(camera.up.length()*view_vector.length())).acos();
        max_angle_up -= 0.001;
        if angle > max_angle_up { angle=max_angle_up; }
        
        let mut max_angle_down = -(((-camera.up).dot(view_vector)/(camera.up.length()*view_vector.length())).acos());
        max_angle_down += 0.001;
        if angle < max_angle_down { angle = max_angle_down;}
    }
    view_vector = rotate_vector_axis_angle(view_vector, get_camera_right(camera), angle);
    if rotate_around_target {
        assert!(false);
        camera.position = camera.target-view_vector;
    }
    else {
        camera.target = camera.position+view_vector;
    }
    if rotate_up {
        camera.up = rotate_vector_axis_angle(camera.up, get_camera_right(camera), angle);
    }
}

pub fn camera_roll(camera: &mut Camera3D, angle: f32) {
    camera.up = rotate_vector_axis_angle(camera.up, get_camera_forward(camera), angle);
}

pub fn update_camera(camera: &mut Camera3D) {
    let lock_view = true;
    let rotate_around_target = false;
    let rotate_up = true;
    let move_in_world_plane = false;
    let dt = get_frame_time();
    let camera_move_speed = dt;
    let camera_rotation_speed = dt;
    let camera_pan_speed = dt;
    
    if is_key_down(KeyCode::Down) {
        camera_pitch(camera, -camera_rotation_speed, rotate_around_target, lock_view, rotate_up);
    }
    if is_key_down(KeyCode::Up) {
        camera_pitch(camera, camera_rotation_speed, rotate_around_target, lock_view, rotate_up);
    }
    if is_key_down(KeyCode::Right) {
        camera_yaw(camera, -camera_rotation_speed, rotate_around_target);
    }
    if is_key_down(KeyCode::Left) {
        camera_yaw(camera, camera_rotation_speed, rotate_around_target);
    }
    if is_key_down(KeyCode::Q) {
        camera_roll(camera, -camera_rotation_speed);
    }
    if is_key_down(KeyCode::E) {
        camera_roll(camera, camera_rotation_speed);
    }

    // Mouse pan
    if is_mouse_button_down(MouseButton::Left) {
        let mouse_delta: Vec2 = mouse_delta_position()/screen_width()*700.;
        if mouse_delta.x > 0.0 {
            camera_move_right(camera, camera_pan_speed, move_in_world_plane);
        }
        if mouse_delta.x < 0.0 {
            camera_move_right(camera, -camera_pan_speed, move_in_world_plane);
        }
        if mouse_delta.y > 0.0 {
            camera_move_up(camera, -camera_pan_speed);
        }
        if mouse_delta.y < 0.0 {
            camera_move_up(camera, camera_pan_speed);
        }
    }
    else {
        let mouse_delta: Vec2 = mouse_delta_position()/screen_width()*700.;
        camera_yaw(camera, -mouse_delta.x, rotate_around_target);
        camera_pitch(camera, -mouse_delta.y, rotate_around_target, lock_view, rotate_up);
    }

    // WASD movement
    if is_key_down(KeyCode::W) {
        camera_move_forward(camera, camera_move_speed, move_in_world_plane);
    }
    if is_key_down(KeyCode::A) {
        camera_move_right(camera, -camera_move_speed, move_in_world_plane);
    }
    if is_key_down(KeyCode::S) {
        camera_move_forward(camera, -camera_move_speed, move_in_world_plane);
    }
    if is_key_down(KeyCode::D) {
        camera_move_right(camera, camera_move_speed, move_in_world_plane);
    }

    if is_key_down(KeyCode::Space) {
        camera_move_up(camera, camera_move_speed);
    }
    if is_key_down(KeyCode::LeftControl) {
        camera_move_up(camera, -camera_move_speed);
    }

    // Mouse wheel zoom
    camera_move_to_target(camera, -mouse_wheel().1);

    // Numpad zoom
    if is_key_pressed(KeyCode::KpSubtract) {
        camera_move_to_target(camera, 2.0);
    }
    if is_key_pressed(KeyCode::KpAdd) {
        camera_move_to_target(camera, -2.0);
    }
}

/*    if (IsKeyDown(KEY_DOWN)) CameraPitch(camera, -cameraRotationSpeed, lockView, rotateAroundTarget, rotateUp);
        if (IsKeyDown(KEY_UP)) CameraPitch(camera, cameraRotationSpeed, lockView, rotateAroundTarget, rotateUp);
        if (IsKeyDown(KEY_RIGHT)) CameraYaw(camera, -cameraRotationSpeed, rotateAroundTarget);
        if (IsKeyDown(KEY_LEFT)) CameraYaw(camera, cameraRotationSpeed, rotateAroundTarget);
        if (IsKeyDown(KEY_Q)) CameraRoll(camera, -cameraRotationSpeed);
        if (IsKeyDown(KEY_E)) CameraRoll(camera, cameraRotationSpeed);
        if is_mouse_button_down(MouseButton::Left) {
            const Vector2 mouseDelta = GetMouseDelta();
            if (mouseDelta.x > 0.0f) CameraMoveRight(camera, cameraPanSpeed, moveInWorldPlane);
            if (mouseDelta.x < 0.0f) CameraMoveRight(camera, -cameraPanSpeed, moveInWorldPlane);
            if (mouseDelta.y > 0.0f) CameraMoveUp(camera, -cameraPanSpeed);
            if (mouseDelta.y < 0.0f) CameraMoveUp(camera, cameraPanSpeed);
        }

        if (IsKeyDown(KEY_W)) CameraMoveForward(camera, cameraMoveSpeed, moveInWorldPlane);
        if (IsKeyDown(KEY_A)) CameraMoveRight(camera, -cameraMoveSpeed, moveInWorldPlane);
        if (IsKeyDown(KEY_S)) CameraMoveForward(camera, -cameraMoveSpeed, moveInWorldPlane);
        if (IsKeyDown(KEY_D)) CameraMoveRight(camera, cameraMoveSpeed, moveInWorldPlane);

        if (mode == CAMERA_FREE)
        {
            if (IsKeyDown(KEY_SPACE)) CameraMoveUp(camera, cameraMoveSpeed);
            if (IsKeyDown(KEY_LEFT_CONTROL)) CameraMoveUp(camera, -cameraMoveSpeed);
        }

        CameraMoveToTarget(camera, -GetMouseWheelMove());
        if (IsKeyPressed(KEY_KP_SUBTRACT)) CameraMoveToTarget(camera, 2.0f);
        if (IsKeyPressed(KEY_KP_ADD)) CameraMoveToTarget(camera, -2.0f); */

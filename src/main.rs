use macroquad::prelude::*;
use libnoise::prelude::*;
pub mod terrain;
pub use crate::terrain::*;

use macroquad::miniquad::*;

#[repr(C)]
struct Vertex2 {
    pos: Vec2,
}

struct Stage<'a> {
    ctx: Box<&'a mut dyn RenderingBackend>,

    pipeline: Pipeline,
    bindings: Bindings,
}

impl<'a> Stage<'a> {
    pub fn new() -> Stage<'a> {
        let mut ctx = Box::new((unsafe {macroquad::window::get_internal_gl().quad_context }));

        #[rustfmt::skip]
        let vertices: [Vertex2; 4] = [
            Vertex2 { pos : Vec2 { x: -0.5, y: -0.5 }},
            Vertex2 { pos : Vec2 { x:  0.5, y: -0.5 }},
            Vertex2 { pos : Vec2 { x:  0.5, y:  0.5 }},
            Vertex2 { pos : Vec2 { x: -0.5, y:  0.5 }},
        ];
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices: [u16; 6] = [0, 1, 2, 0, 2, 3];
        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![],
        };

        let shader = ctx
            .new_shader(
                match ctx.info().backend {
                    Backend::OpenGl => ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                    Backend::Metal => todo!(),
                },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float2),
            ],
            shader,
            PipelineParams::default(),
        );

        Stage {
            pipeline,
            bindings,
            ctx,
        }
    }
}

impl<'a> Stage<'a> {
    fn update(&mut self) {}

    fn draw(&mut self, camera: &Camera3D) {
        let t = date::now();

        self.ctx.begin_default_pass(Default::default());

        self.ctx.apply_pipeline(&self.pipeline);
        self.ctx.apply_bindings(&self.bindings);
        for i in 0..10 {
            let t = t + i as f64 * 0.3;

            self.ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    offset: (t.sin() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5),
                    projection: camera.matrix(),
                    model: Mat4::IDENTITY,
                }));
            self.ctx.draw(0, 6, 1);
        }
        self.ctx.end_render_pass();
    }
}

#[macroquad::main("Texture")]
async fn main() {
    let mut camera = Camera3D {
        position: vec3(2., 2., 2.),
        up: vec3(0., 1., 0.),
        target: vec3(0., 0., 0.),
        ..Default::default()
    };
    let camera_speed = 0.3;
    let mesh = gen_terrain_mesh(10, 10);
    let mut stage = Stage::new();
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
        draw_mesh(&mesh);
        draw_grid(20, 0.1, BLACK, GRAY);
        stage.draw(&camera);
        draw_cube_wires(vec3(0., 0., 0.), vec3(1., 1., 1.), DARKGREEN);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);
        
        next_frame().await
    }
}

mod shader {
    use macroquad::miniquad::*;
    use macroquad::prelude::*;
    
    pub const VERTEX: &str = r#"
    #version 330
    layout (location = 0) in vec2 in_pos;

    uniform vec2 offset;
    uniform mat4 model;
    uniform mat4 projection;

    out vec2 pos;

    void main() {
        gl_Position = projection*model*vec4(in_pos + offset, 0, 1);
        pos = in_pos+offset;
    }"#;

    pub const FRAGMENT: &str = r#"#version 330
    in vec2 pos;
    out vec4 FragColor;
    void main() {
        FragColor = vec4(dot(pos, pos), pos.x, pos.y, 1.0);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("offset", UniformType::Float2),
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("projection", UniformType::Mat4),
                    ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32),
        pub model: Mat4,
        pub projection: Mat4,
    }
}

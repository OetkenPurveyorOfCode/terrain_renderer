use macroquad::miniquad::*;
use macroquad::prelude::*;

#[repr(C)]
struct Vertex2 {
    pos: Vec2,
}

pub struct Heightmap {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl Heightmap {
    pub fn new() -> Heightmap {
        let ctx = Box::new(unsafe {macroquad::window::get_internal_gl().quad_context });

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

        Heightmap {
            pipeline,
            bindings,
        }
    }

    pub fn draw(&mut self, camera: &Camera3D) {
        let ctx = Box::new(unsafe {macroquad::window::get_internal_gl().quad_context });
        let t = date::now();

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);
        for i in 0..10 {
            let t = t + i as f64 * 0.3;

            ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    offset: (t.sin() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5),
                    projection: camera.matrix(),
                    model: Mat4::IDENTITY,
                }));
            ctx.draw(0, 6, 1);
        }
        ctx.end_render_pass();
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


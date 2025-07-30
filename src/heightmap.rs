use macroquad::miniquad::*;
use macroquad::prelude::*;

pub struct Heightmap {
    pipeline: Pipeline,
    bindings: Bindings,
    indices_len: i32,
}

use libnoise::prelude::*;

fn gen_terrain_mesh(x_divisions: usize, y_divisions: usize) -> Mesh {
    let generator = Source::simplex(rand::rand() as u64).fbm(5, 0.013, 2.0, 0.5);
    let mut vertices = Vec::with_capacity(x_divisions*y_divisions);
    let mut indices = Vec::with_capacity(8*x_divisions*y_divisions);
    for xi in 0..x_divisions {
        for yi in 0..y_divisions {
            let (x, y) = (xi as f32/x_divisions as f32, yi as f32/y_divisions as f32);
            let (x, y) = (2.*x-1., 2.*y-1.);
            let height = generator.sample([x as f64*100.,y as f64*100.]) as f32;
            dbg!(height);
            vertices.push(Vertex::new(x, height, y, 0.0, 0.0, Color::new(height, height, height, 1.0)));
        }
    }
    for xi in 0..x_divisions-1 {
        for yi in 0..y_divisions-1 {
            let index: u16 = (xi*x_divisions+yi).try_into().unwrap();
            let next_x_index: u16 = ((xi+1)*x_divisions+yi).try_into().unwrap();
            let next_y_index: u16 = (xi*x_divisions+yi+1).try_into().unwrap();
            let next_xy_index: u16 = ((xi+1)*x_divisions+yi+1).try_into().unwrap();
            indices.extend([index, next_x_index, next_xy_index].iter());
            indices.extend([index, next_xy_index, next_y_index].iter());
        }
    }
    return Mesh{
        vertices: vertices,
        indices: indices,
        texture: None,
    }
}

impl Heightmap {
    pub fn new() -> Heightmap {
        let ctx = Box::new(unsafe {macroquad::window::get_internal_gl().quad_context });
        let mesh = gen_terrain_mesh(10, 10);
        #[rustfmt::skip]
        let vertices = mesh.vertices;
        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );

        let indices = mesh.indices;
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
            .new_shader(ShaderSource::Glsl {
                        vertex: shader::VERTEX,
                        fragment: shader::FRAGMENT,
                    },
                shader::meta(),
            )
            .unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[
                VertexAttribute::new("in_pos", VertexFormat::Float3),
            ],
            shader,
            PipelineParams::default(),
        );

        Heightmap {
            pipeline,
            bindings,
            indices_len: indices.len() as i32,
        }
    }

    pub fn draw(&mut self, camera: &Camera3D) {
        let ctx = Box::new(unsafe {macroquad::window::get_internal_gl().quad_context });
        let t = date::now();

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

            ctx
                .apply_uniforms(UniformsSource::table(&shader::Uniforms {
                    offset: (t.sin() as f32 * 0.5, (t * 3.).cos() as f32 * 0.5, 0.0),
                    projection: camera.matrix(),
                    model: Mat4::IDENTITY,
                }));
            ctx.draw(0, self.indices_len, 1);
        ctx.end_render_pass();
    }
}

mod shader {
    use macroquad::miniquad::*;
    use macroquad::prelude::*;
    
    pub const VERTEX: &str = r#"
    #version 330
    layout (location = 0) in vec3 in_pos;

    uniform vec3 offset;
    uniform mat4 model;
    uniform mat4 projection;

    out vec2 pos;

    void main() {
        gl_Position = projection*model*vec4(in_pos, 1);
        pos = (in_pos+offset).xy;
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
                    UniformDesc::new("offset", UniformType::Float3),
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("projection", UniformType::Mat4),
                    ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub offset: (f32, f32, f32),
        pub model: Mat4,
        pub projection: Mat4,
    }
}


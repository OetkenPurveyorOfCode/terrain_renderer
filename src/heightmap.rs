use macroquad::miniquad::*;
use macroquad::prelude::*;

pub struct Heightmap {
    pipeline: Pipeline,
    bindings: Bindings,
    indices_len: i32,
}

use libnoise::prelude::*;

#[repr(C)]
struct Vertex {
    pos: Vec3,
    uv: Vec2,
    color: Vec4,
    normal: Vec3,
}

impl Heightmap {
    pub fn new() -> Heightmap {
        let ctx = Box::new(unsafe { macroquad::window::get_internal_gl().quad_context });
        let (x_divisions, y_divisions) = (30, 30);
        let generator = Source::simplex(rand::rand() as u64).fbm(5, 0.013, 2.0, 0.5);
        let mut vertices = Vec::with_capacity(x_divisions * y_divisions);
        for xi in 0..x_divisions {
            for yi in 0..y_divisions {
                let (u, v) = (
                    xi as f32 / x_divisions as f32,
                    yi as f32 / y_divisions as f32,
                );
                let (x, y) = (2. * u - 1., 2. * v - 1.);
                let height =
                    0.5 * (generator.sample([x as f64 * 100., y as f64 * 100.]) as f32 + 1.0);
                vertices.push(Vertex {
                    pos: Vec3::new(x, height, y),
                    uv: Vec2::new(u, v),
                    color: Vec4::new(1.0, height, height, 1.0),
                    normal: Vec3::ZERO,
                });
            }
        }



        let mut indices: Vec<u32> = Vec::with_capacity(6 * x_divisions * y_divisions);
        for xi in 0..x_divisions - 1 {
            for yi in 0..y_divisions - 1 {
                let index: u32 = (xi * x_divisions + yi).try_into().unwrap();
                let next_x_index: u32 = ((xi + 1) * x_divisions + yi).try_into().unwrap();
                let next_y_index: u32 = (xi * x_divisions + yi + 1).try_into().unwrap();
                let next_xy_index: u32 = ((xi + 1) * x_divisions + yi + 1).try_into().unwrap();
                indices.extend([index, next_x_index, next_xy_index].iter());
                indices.extend([index, next_xy_index, next_y_index].iter());
                let v0 = &vertices[index as usize];
                let v1 = &vertices[next_x_index as usize];
                let v2 = &vertices[next_xy_index as usize];
                let normal = (v1.pos-v0.pos).cross(v2.pos-v0.pos).normalize();
                vertices[index as usize].normal += normal;
                vertices[next_x_index as usize].normal += normal;
                vertices[next_xy_index as usize].normal += normal;

                let v0 = &vertices[index as usize];
                let v1 = &vertices[next_xy_index as usize];
                let v2 = &vertices[next_y_index as usize];
                let normal = (v1.pos-v0.pos).cross(v2.pos-v0.pos).normalize();
                vertices[index as usize].normal += normal;
                vertices[next_xy_index as usize].normal += normal;
                vertices[next_y_index as usize].normal += normal;
            }
        }
        for vertex in &mut vertices {
            vertex.normal = vertex.normal.normalize();
        }

        let vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&vertices),
        );


        let index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let grass_image = Image::from_file_with_format(
            include_bytes!("../assets/grass.png"),
            None,
        ).unwrap();

        let grass_texture_id = ctx.new_texture(
            TextureAccess::Static,
            TextureSource::Bytes(&grass_image.bytes),
            TextureParams{
                kind: TextureKind::Texture2D,
                format: TextureFormat::RGBA8,
                wrap: TextureWrap::Clamp,
                min_filter: FilterMode::Linear,
                mag_filter: FilterMode::Linear,
                mipmap_filter: MipmapFilterMode::Linear,
                width: grass_image.width as u32,
                height: grass_image.height as u32,
                allocate_mipmaps: true,
                sample_count: 1,
            }
        );
        ctx.texture_generate_mipmaps(grass_texture_id);

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: vec![grass_texture_id],
        };

        let shader = ctx
            .new_shader(
                ShaderSource::Glsl {
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
                VertexAttribute::new("in_uv", VertexFormat::Float2),
                VertexAttribute::new("in_color", VertexFormat::Float4),
                VertexAttribute::new("in_normal", VertexFormat::Float3),
            ],
            shader,
            PipelineParams {
                cull_face: CullFace::Nothing,
                depth_test: Comparison::Less,
                depth_write: true,
                ..Default::default()
            },
        );

        Heightmap {
            pipeline,
            bindings,
            indices_len: indices.len() as i32,
        }
    }

    pub fn draw(&mut self, camera: &Camera3D) {
        let ctx = Box::new(unsafe { macroquad::window::get_internal_gl().quad_context });
        let t = date::now();

        ctx.begin_default_pass(Default::default());

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms {
            projection: camera.matrix(),
            model: Mat4::IDENTITY,
            light_dir: -camera.position.normalize(),
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
    layout (location = 1) in vec2 in_uv;
    layout (location = 2) in vec4 in_color;
    layout (location = 3) in vec3 in_normal;

    uniform mat4 model;
    uniform mat4 projection;

    out vec4 color;
    out vec3 normal;
    out vec2 texcoord;
    void main() {
        gl_Position = projection*model*vec4(in_pos, 1);
        color = vec4(in_pos.y, in_pos.y, in_pos.y, 1.0);
        normal = in_normal;
        texcoord = in_uv;
    }"#;

    pub const FRAGMENT: &str = r#"#version 330
    in vec4 color;
    in vec3 normal;
    in vec2 texcoord;

    uniform vec3 light_dir = vec3(1.0, 0.0, 0.0);
    uniform sampler2D terrain_texture;

    out vec4 FragColor;

    void main() {
        float diffuse = dot(light_dir, normalize(normal));
        diffuse = max(0.3, diffuse);
        FragColor = diffuse*texture(terrain_texture, texcoord);
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["terrain_texture".to_string()],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("model", UniformType::Mat4),
                    UniformDesc::new("projection", UniformType::Mat4),
                    UniformDesc::new("light_dir", UniformType::Float3),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        pub model: Mat4,
        pub projection: Mat4,
        pub light_dir: Vec3,
    }
}

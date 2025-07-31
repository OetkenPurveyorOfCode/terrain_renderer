use macroquad::miniquad::*;
use macroquad::prelude::*;
use libnoise::prelude::*;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Chunk {
    offset: Vec2,
    bindings: Bindings,
    indices_len: i32,
}

impl Chunk {
    pub fn new<T: Generator<2>>(generator: &T, offset: Vec2, texture_ids: Vec<TextureId>, divisions: (usize, usize), terrain_scale: f64) -> Chunk {
        let ctx = Box::new(unsafe { macroquad::window::get_internal_gl().quad_context });
        let (x_divisions, y_divisions) = divisions;
        let mut vertices = Vec::with_capacity(x_divisions * y_divisions);
        for xi in 0..x_divisions {
            for yi in 0..y_divisions {
                let (u, v, next_u, next_v, prev_u, prev_v) = (
                    xi as f32 / (x_divisions-1) as f32,
                    yi as f32 / (y_divisions-1) as f32,
                    (xi as f32 +1.)/ (x_divisions-1) as f32,
                    (yi as f32 +1.) / (y_divisions-1) as f32,
                    (xi as f32 -1.) / (x_divisions-1) as f32,
                    (yi as f32 -1.) / (y_divisions-1) as f32,
                );
                //let (x, y) = (2. * u - 1., 2. * v - 1.);
                let (x, y, next_x, next_y, prev_x, prev_y) = (
                    u+offset.x,
                    v+offset.y,
                    next_u+offset.x,
                    next_v+offset.y,
                    prev_u+offset.x,
                    prev_v+offset.y,                   
                );
                let (height, next_x_height, next_y_height, prev_x_height, prev_y_height) = (
                0.5 * (generator.sample([x as f64 * terrain_scale, y as f64 * terrain_scale]) as f32 + 1.0),
                0.5 * (generator.sample([next_x as f64 * terrain_scale, y as f64 * terrain_scale]) as f32 + 1.0),
                0.5 * (generator.sample([x as f64 * terrain_scale, next_y as f64 * terrain_scale]) as f32 + 1.0),
                0.5 * (generator.sample([prev_x as f64 * terrain_scale, y as f64 * terrain_scale]) as f32 + 1.0),
                0.5 * (generator.sample([x as f64 * terrain_scale, prev_y as f64 * terrain_scale]) as f32 + 1.0));
                let (pos, next_x_pos, next_y_pos, prev_x_pos, prev_y_pos) = (
                    Vec3::new(x, height, y),
                    Vec3::new(next_x, next_x_height, y),
                    Vec3::new(x, next_y_height, next_y),
                    Vec3::new(prev_x, prev_x_height, y),
                    Vec3::new(x, prev_y_height, prev_y),
                );
                let normal1 = (next_x_pos-pos).cross(next_y_pos-pos).normalize();
                let normal2 = (prev_x_pos-pos).cross(prev_y_pos-pos).normalize();
                let normal = (normal1 + normal2).normalize();
                vertices.push(Vertex {
                    pos: Vec3::new(x, height, y),
                    uv: Vec2::new(u, v),
                    color: Vec4::new(1.0, height, height, 1.0),
                    normal: normal,
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
            }
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

        let bindings = Bindings {
            vertex_buffers: vec![vertex_buffer],
            index_buffer: index_buffer,
            images: texture_ids,
        };
        Chunk {offset: offset, bindings: bindings, indices_len: indices.len() as i32}
    }

    fn draw(&mut self, pipeline: &Pipeline, camera: &Camera3D, light_dir: Vec3) {
        let ctx = Box::new(unsafe { macroquad::window::get_internal_gl().quad_context });

        ctx.apply_pipeline(&pipeline);
        ctx.apply_bindings(&self.bindings);

        ctx.apply_uniforms(UniformsSource::table(&shader::Uniforms {
            projection: camera.matrix(),
            model: Mat4::IDENTITY,
            light_dir: -light_dir.normalize(),
        }));
        ctx.draw(0, self.indices_len, 1);
        ctx.end_render_pass();
    }
}

impl Drop for Chunk {
    fn drop(&mut self) {
        let ctx = Box::new(unsafe { macroquad::window::get_internal_gl().quad_context });
        for vertex_buffer in &mut self.bindings.vertex_buffers {
            ctx.delete_buffer(*vertex_buffer);
        }
        ctx.delete_buffer(self.bindings.index_buffer);
    }
}


#[repr(C)]
struct Vertex {
    pos: Vec3,
    uv: Vec2,
    color: Vec4,
    normal: Vec3,
}

pub struct Heightmap<T: Generator<2>> {
    pub pipeline: Pipeline,
    pub generator: T,
    pub textures: Vec<TextureId>,
    pub chunks: HashMap<IVec2, Chunk>,
    pub divisions: (usize, usize),
    pub terrain_scale: f64,
}


impl<T: Generator<2>> Heightmap<T> {
    pub fn new(generator: T, divisions: (usize, usize), terrain_scale: f64) -> Heightmap<T> {
        let ctx = Box::new(unsafe { macroquad::window::get_internal_gl().quad_context });

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
        let textures = load_textures();
        Heightmap {
            pipeline,
            generator,
            textures,
            chunks: HashMap::new(),
            divisions,
            terrain_scale
        }
    }

    pub fn draw(&mut self, camera: &Camera3D, light_dir: Vec3) {
        // generate chunks around camera position
        let camera_offset = camera.position.floor();
        let camera_offset = IVec2::new(camera_offset.x as i32, camera_offset.z as i32);
        let mut added = 0;
        for x in -10..10 {
            for y in -10..10 {
                if added > 1 { break; }
                let offset = camera_offset+IVec2::new(x,y);
                self.chunks.entry(offset).or_insert_with(|| {
                    added += 1;
                    Chunk::new(&self.generator, Vec2::new(offset.x as f32, offset.y as f32), self.textures.clone(), self.divisions, self.terrain_scale)
                });
            }
        }
        if added >= 1 { dbg!(added);}
        if self.chunks.len() >= 600 {
            
            self.chunks.retain(|key, _| (*key-camera_offset).length_squared() < 200);
        }

        for (_, chunk) in &mut self.chunks {
            chunk.draw(&self.pipeline, camera, light_dir);
        }
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

    out vec3 pos;
    out vec4 color;
    out vec3 normal;
    out vec2 texcoord;
    void main() {
        gl_Position = projection*model*vec4(in_pos, 1);
        color = vec4(in_pos.y, in_pos.y, in_pos.y, 1.0);
        pos = in_pos;
        normal = in_normal;
        texcoord = in_uv;
    }"#;

    pub const FRAGMENT: &str = r#"
    #version 330
    in vec3 pos;
    in vec4 color;
    in vec3 normal;
    in vec2 texcoord;

    uniform vec3 light_dir = vec3(1.0, 0.0, 0.0);
    uniform sampler2D snow_texture;
    uniform sampler2D grass_texture;
    uniform sampler2D rock_texture;
    uniform sampler2D dirt_texture;

    out vec4 FragColor;

    void main() {
        float diffuse = dot(light_dir, normalize(normal));
        diffuse = max(0.3, diffuse);
        vec4 texcolor;
        if (pos.y < 0.25) {
            texcolor = texture(dirt_texture, texcoord);
        }
        else if (0.25 <= pos.y && pos.y < 0.5) {
            float t = 4.*(pos.y-0.25);
            texcolor = mix(texture(dirt_texture, texcoord), texture(grass_texture, texcoord), t);
        }
        else if (0.5 <= pos.y && pos.y < 0.75) {
            float t = 4.*(pos.y-0.5);
            texcolor = mix(texture(grass_texture, texcoord), texture(rock_texture, texcoord), t);
        }
        else {
            float t = 4.*(pos.y-0.75);
            texcolor = mix(texture(rock_texture, texcoord), texture(snow_texture, texcoord), t);
        }
        FragColor = diffuse*texcolor ;
    }"#;

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec!["grass_texture".to_string(), "snow_texture".to_string(), "rock_texture".to_string(), "dirt_texture".to_string()],
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

fn texture_from_png(ctx: &mut Box<&mut dyn RenderingBackend>, bytes: &[u8]) -> TextureId {
    let image = Image::from_file_with_format(
        bytes,
        None,
    ).unwrap();

    let texture_id = ctx.new_texture(
        TextureAccess::Static,
        TextureSource::Bytes(&image.bytes),
        TextureParams{
            kind: TextureKind::Texture2D,
            format: TextureFormat::RGBA8,
            wrap: TextureWrap::Clamp,
            min_filter: FilterMode::Linear,
            mag_filter: FilterMode::Linear,
            mipmap_filter: MipmapFilterMode::Linear,
            width: image.width as u32,
            height: image.height as u32,
            allocate_mipmaps: true,
            sample_count: 1,
        }
    );
    ctx.texture_generate_mipmaps(texture_id);
    return texture_id;
}

pub fn load_textures() -> Vec<TextureId> {
    let mut ctx = Box::new(unsafe { macroquad::window::get_internal_gl().quad_context });
    vec![texture_from_png(&mut ctx, include_bytes!("../assets/grass.png")),
     texture_from_png(&mut ctx, include_bytes!("../assets/snow.png")),
     texture_from_png(&mut ctx, include_bytes!("../assets/rock.png")),
     texture_from_png(&mut ctx, include_bytes!("../assets/dirt.png"))]
}

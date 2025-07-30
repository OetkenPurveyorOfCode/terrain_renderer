use macroquad::prelude::*;
use libnoise::prelude::*;

pub fn gen_terrain_mesh(x_divisions: usize, y_divisions: usize) -> Mesh {
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

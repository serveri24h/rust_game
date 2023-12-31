use bevy::{
    prelude::*,
    render::render_resource::{PrimitiveTopology, VertexFormat},
    render::mesh::{Indices, MeshVertexAttribute}
};

use crate::HEX_RADIUS;

pub fn create_hex() -> Mesh {
    let s: f32 = (3.0 as f32).sqrt()/2.0; 
    let x_v = vec![ HEX_RADIUS,    HEX_RADIUS/2.0,  -HEX_RADIUS/2.0,   -HEX_RADIUS,  -HEX_RADIUS/2.0,    HEX_RADIUS/2.0,    HEX_RADIUS];
    let y_v = vec![ 0.0,  -s*HEX_RADIUS,   -s*HEX_RADIUS,     0.0,  s*HEX_RADIUS,      s*HEX_RADIUS,    0.0];

    let mut hex = Mesh::new(PrimitiveTopology::TriangleList);
    let mut v_pos = vec![[0.0, 0.0, 0.0]];
    let mut v_nor = vec![[1.0, 1.0, 1.0]];
    let mut v_uv0 = vec![[1.0, 1.0]];
    for i in 0..7 {
        v_pos.push([x_v[i], y_v[i], 0.0]);
        v_nor.push([0.0, 0.0, 0.0]);
        v_uv0.push([0.0, 0.0]);
    }

    let v_color: Vec<u32> = vec![Color::BLACK.as_linear_rgba_u32()];
    //v_color.extend_from_slice(&[Color::YELLOW.as_linear_rgba_u32(); 7]);

    hex.insert_attribute(
        MeshVertexAttribute::new("Vertex_Color", 1, VertexFormat::Uint32),
        v_color,
    );

    hex.insert_attribute(Mesh::ATTRIBUTE_POSITION, v_pos);
    hex.insert_attribute(Mesh::ATTRIBUTE_NORMAL, v_nor);
    hex.insert_attribute(Mesh::ATTRIBUTE_UV_0, v_uv0);

    let mut indices = vec![0, 1, 7];
    for i in 2..=10 {
        indices.extend_from_slice(&[0, i, i - 1]);
    }
    hex.set_indices(Some(Indices::U32(indices)));
    hex
}

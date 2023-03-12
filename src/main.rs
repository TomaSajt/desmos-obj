use std::{
    collections::HashMap,
    fs::File,
    hash::{Hash, Hasher},
    io::BufReader,
};

use anyhow::Result;

fn main() -> Result<()> {
    let (indices, vertices) = load_model("resources/model.obj")?;
    let mut faces: Vec<(u32, u32, u32)> = Vec::new();
    for i in 0..indices.len() / 3 {
        faces.push((indices[3 * i], indices[3 * i + 1], indices[3 * i + 2]));
    }
    let v_x = to_desmos_array(vertices.iter().map(|v| v.pos.0));
    let v_y = to_desmos_array(vertices.iter().map(|v| v.pos.1));
    let v_z = to_desmos_array(vertices.iter().map(|v| v.pos.2));
    let f_a = to_desmos_array(faces.iter().map(|f| f.0 + 1));
    let f_b = to_desmos_array(faces.iter().map(|f| f.1 + 1));
    let f_c = to_desmos_array(faces.iter().map(|f| f.2 + 1));

    println!("V_{{X}}={}", v_x);
    println!("V_{{Y}}={}", v_y);
    println!("V_{{Z}}={}", v_z);
    println!("F_{{A}}={}", f_a);
    println!("F_{{B}}={}", f_b);
    println!("F_{{C}}={}", f_c);
    Ok(())
}
fn to_desmos_array<T: ToString>(vals: impl IntoIterator<Item = T>) -> String {
    format!(
        "\\left[{}\\right]",
        vals.into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    )
}

#[derive(Copy, Clone, Debug)]
struct Vertex {
    pos: (f32, f32, f32),
    texcoord: (f32, f32),
}

impl PartialEq for Vertex {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.texcoord == other.texcoord
    }
}

impl Eq for Vertex {}

impl Hash for Vertex {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.0.to_bits().hash(state);
        self.pos.1.to_bits().hash(state);
        self.pos.2.to_bits().hash(state);
        self.texcoord.0.to_bits().hash(state);
        self.texcoord.1.to_bits().hash(state);
    }
}

fn load_model(path: &str) -> Result<(Vec<u32>, Vec<Vertex>)> {
    let mut reader = BufReader::new(File::open(path)?);

    let (models, _) = tobj::load_obj_buf(
        &mut reader,
        &tobj::LoadOptions {
            triangulate: true,
            single_index: true,
            ..Default::default()
        },
        |_| Ok(Default::default()),
    )?;

    let mut unique_vertices = HashMap::new();
    let mut indices = Vec::new();
    let mut vertices = Vec::new();
    for model in &models {
        for ind in &model.mesh.indices {
            let pos_offset = (3 * ind) as usize;
            let texcoord_offset = (2 * ind) as usize;
            let vertex = Vertex {
                pos: (
                    model.mesh.positions[pos_offset],
                    model.mesh.positions[pos_offset + 1],
                    model.mesh.positions[pos_offset + 2],
                ),
                texcoord: (
                    model.mesh.texcoords[texcoord_offset],
                    model.mesh.texcoords[texcoord_offset + 1],
                ),
            };
            if let Some(index) = unique_vertices.get(&vertex) {
                indices.push(*index as u32);
            } else {
                let index = vertices.len();
                unique_vertices.insert(vertex, index);
                vertices.push(vertex);
                indices.push(index as u32);
            }
        }
    }

    Ok((indices, vertices))
}

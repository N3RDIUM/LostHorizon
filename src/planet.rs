use bevy::{
    prelude::*,
    render::{
        mesh::Indices,
        render_asset::RenderAssetUsages,
        render_resource::PrimitiveTopology,
    },
};

#[derive(Component)]
pub struct Planet {
}

impl Planet {
    pub fn default() -> Planet {
        Planet { }
    }

    pub fn create_mesh(self) -> Mesh {
        let mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, 0.5, 0.5],
                [-0.5, 0.5, 0.5],
                // bottom   (-y)
                [-0.5, -0.5, -0.5],
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [-0.5, -0.5, 0.5],
                // right    (+x)
                [0.5, -0.5, -0.5],
                [0.5, -0.5, 0.5],
                [0.5, 0.5, 0.5],
                [0.5, 0.5, -0.5],
                // left     (-x)
                [-0.5, -0.5, -0.5],
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [-0.5, 0.5, -0.5],
                // back     (+z)
                [-0.5, -0.5, 0.5],
                [-0.5, 0.5, 0.5],
                [0.5, 0.5, 0.5],
                [0.5, -0.5, 0.5],
                // forward  (-z)
                [-0.5, -0.5, -0.5],
                [-0.5, 0.5, -0.5],
                [0.5, 0.5, -0.5],
                [0.5, -0.5, -0.5],
            ],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![
                [0.0, 0.2],
                [0.0, 0.0],
                [1.0, 0.0],
                [1.0, 0.2],
                // Assigning the UV coords for the bottom side.
                [0.0, 0.45],
                [0.0, 0.25],
                [1.0, 0.25],
                [1.0, 0.45],
                // Assigning the UV coords for the right side.
                [1.0, 0.45],
                [0.0, 0.45],
                [0.0, 0.2],
                [1.0, 0.2],
                // Assigning the UV coords for the left side.
                [1.0, 0.45],
                [0.0, 0.45],
                [0.0, 0.2],
                [1.0, 0.2],
                // Assigning the UV coords for the back side.
                [0.0, 0.45],
                [0.0, 0.2],
                [1.0, 0.2],
                [1.0, 0.45],
                // Assigning the UV coords for the forward side.
                [0.0, 0.45],
                [0.0, 0.2],
                [1.0, 0.2],
                [1.0, 0.45],
            ],
        )
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_NORMAL,
            vec![
                // Normals for the top side (towards +y)
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                [0.0, 1.0, 0.0],
                // Normals for the bottom side (towards -y)
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                [0.0, -1.0, 0.0],
                // Normals for the right side (towards +x)
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                [1.0, 0.0, 0.0],
                // Normals for the left side (towards -x)
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                [-1.0, 0.0, 0.0],
                // Normals for the back side (towards +z)
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                [0.0, 0.0, 1.0],
                // Normals for the forward side (towards -z)
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
                [0.0, 0.0, -1.0],
            ],
        )
        .with_inserted_indices(Indices::U32(vec![
            0, 3, 1, 1, 3, 2, // triangles making up the top (+y) facing side.
            4, 5, 7, 5, 6, 7, // bottom (-y)
            8, 11, 9, 9, 11, 10, // right (+x)
            12, 13, 15, 13, 14, 15, // left (-x)
            16, 19, 17, 17, 19, 18, // back (+z)
            20, 21, 23, 21, 22, 23, // forward (-z)
        ]));

        return mesh;
    }
}

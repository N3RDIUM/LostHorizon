use bevy::prelude::*;
use crate::planet::Planet;
use crate::planet::octree::Leaf;
use marching_cubes::marching::{GridCell, MarchingCubes, Triangle};

// For stitching different LoD levels together:
// Just check whether the current node is
// an edge to a lower level. If it is, extend its
// bounds by a few points.

// TODO: struct Mesh

pub fn build_mesh(_planet: Planet, _leaf: Leaf) {
    let grid = GridCell {
        positions: [ // TODO: Generate grid
            [0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0],
            [1.0, 1.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
            [1.0, 0.0, 1.0],
            [1.0, 1.0, 1.0],
            [0.0, 1.0, 1.0],
        ],
        value: [0.0, 0.5, 0.5, 1.0, 0.0, 1.0, 1.0, 0.0],
    };
    let mut triangles = vec![];

    let isolevel = 0.5;
    let mc = MarchingCubes::new(isolevel, grid);
    let triangle_count = mc.polygonise(&mut triangles);

    assert_eq!(triangle_count, 4);
}

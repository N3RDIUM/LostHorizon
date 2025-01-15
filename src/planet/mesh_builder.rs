// use bevy::prelude::*;

pub struct MeshBuilder { }

// For stitching different LoD levels together:
// Just check whether the current node is
// an edge to a lower level. If it is, extend its
// bounds by a few points.

impl MeshBuilder {
    pub fn new() -> MeshBuilder {
        MeshBuilder { }
    }
}


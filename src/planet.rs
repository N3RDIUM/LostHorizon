use bevy::prelude::*;
mod mesh_builder;
pub mod octree;

#[derive(Component)]
pub struct Planet {
    radius: f32,
    octree: octree::Node,
}

impl Planet {
    pub fn new() -> Planet {
        let octree = octree::Node::new(octree::NodeBounds {
            min: Vec3::new(-1.0, -1.0, -1.0),
            max: Vec3::new(1.0, 1.0, 1.0),
        });

        Planet {
            radius: 1.0,
            octree,
        }
    }

    pub fn noise_function(self, point: Vec3) -> f32 {
        let mut noise = 1.0;
        let distance = Vec3::ZERO.distance(point);

        if distance > self.radius {
            noise = 0.0
        }

        return noise;
    }

    pub fn update_octree(mut self, cameras: Vec<Vec3>) {
        self.octree.update(cameras);
    }
}

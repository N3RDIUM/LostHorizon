use bevy::prelude::*;

#[derive(Component)]
pub struct Planet {
    radius: f32
}

impl Planet {
    pub fn new() -> Planet {
        Planet {
            radius: 1.0
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
}

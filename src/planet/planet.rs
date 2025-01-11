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

    pub fn noise_function(self, point: [f32; 3]) -> f32 {
        let [x, y, z] = point;

        // TODO
        // + return multiple noise values:
        //     - One for the actual frequency thing
        //     - One for the material (i.e. texture)
        
        let mut noise = 1.0;
        let distance = (x * x + y * y + z * z).sqrt();

        if distance > self.radius {
            noise = 0.0
        }

        return noise;
    }
}

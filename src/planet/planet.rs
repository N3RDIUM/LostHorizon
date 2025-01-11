use bevy::prelude::*;

#[derive(Component)]
pub struct Planet {}

impl Planet {
    pub fn default() -> Planet {
        Planet {}
    }

    pub fn noise_function(self, point: [f32; 3]) -> f32 {
        let [_x, _y, _z] = point;

        // TODO
        // + return multiple noise values:
        //     - One for the actual frequency thing
        //     - One for the material (i.e. texture)

        return 0.0;
    }
}

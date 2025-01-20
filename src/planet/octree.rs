use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub struct Bounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl Bounds {
    fn get_centre(&self) -> Vec3 {
        return self.min.midpoint(self.max);
    }

    fn get_size(&self) -> f32 {
        return self.min.distance(self.max);
    }
}

enum State {
    UNSPLIT,
    SPLIT,
}

pub struct Node {
    bounds: Bounds,
    _child: Leaf,
    children: Vec<Arc<Mutex<Node>>>,
    state: State,
}

impl Node {
    pub fn new(bounds: Bounds) -> Node {
        Node {
            bounds,
            _child: Leaf::new(),
            children: vec![],
            state: State::UNSPLIT,
        }
    }

    fn get_split_points(bounds: &Bounds) -> [f32; 9] {
        let xmin = bounds.min.x;
        let ymin = bounds.min.y;
        let zmin = bounds.min.z;

        let xmax = bounds.max.x;
        let ymax = bounds.max.y;
        let zmax = bounds.max.z;

        let xmid = (xmin + xmax) / 2.0;
        let ymid = (ymin + ymax) / 2.0;
        let zmid = (zmin + zmax) / 2.0;

        return [xmin, xmid, xmax, ymin, ymid, ymax, zmin, zmid, zmax];
    }

    fn get_child_bounds(bounds: &Bounds) -> [[f32; 6]; 8] {
        let [xmin, xmid, xmax, ymin, ymid, ymax, zmin, zmid, zmax] = Node::get_split_points(bounds);

        return [
            [xmin, xmid, ymin, ymid, zmin, zmid],
            [xmid, xmax, ymin, ymid, zmin, zmid],
            [xmin, xmid, ymid, ymax, zmin, zmid],
            [xmid, xmax, ymid, ymax, zmin, zmid],
            [xmin, xmid, ymin, ymid, zmid, zmax],
            [xmid, xmax, ymin, ymid, zmid, zmax],
            [xmin, xmid, ymid, ymax, zmid, zmax],
            [xmid, xmax, ymid, ymax, zmid, zmax],
        ];
    }

    fn add_child(&mut self, bounds: [f32; 6]) {
        let [xmin, xmax, ymin, ymax, zmin, zmax] = bounds;

        let new_node = Node::new(Bounds {
            min: Vec3 {
                x: xmin,
                y: ymin,
                z: zmin,
            },
            max: Vec3 {
                x: xmax,
                y: ymax,
                z: zmax,
            },
        });

        self.children.push(Arc::new(Mutex::new(new_node)));
    }

    fn split(&mut self) {
        match self.state {
            State::SPLIT => return,
            State::UNSPLIT => {
                let child_bounds = Node::get_child_bounds(&self.bounds);

                for (_, bounds) in child_bounds.iter().enumerate() {
                    self.add_child(*bounds);
                }

                self.state = State::SPLIT;
            }
        }
    }

    fn dispose_children(&mut self) {
        for child in self.children.iter_mut() {
            let mut node = child.lock().unwrap();
            node.unsplit();
        }
        self.children.clear();
    }

    fn unsplit(&mut self) {
        match self.state {
            State::UNSPLIT => return,
            State::SPLIT => {
                self.dispose_children();
                self.state = State::UNSPLIT;
            }
        }
    }

    pub fn update(&mut self, cameras: Vec<Vec3>) {
        let centre = self.bounds.get_centre();
        let threshold = self.bounds.get_size();

        for camera in cameras.iter() {
            let distance = centre.distance(*camera);

            if distance < threshold {
                self.split();
            } else {
                self.unsplit();
            }
        }
    }
}

pub struct Leaf {}

impl Leaf {
    fn new() -> Leaf {
        Leaf {} // Bounds
    }
}

#[cfg(test)]
mod octree_node {
    use super::*;

    #[test]
    fn get_bounds_centre() {
        let bounds = Bounds {
            min: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Vec3 {
                x: 8.0,
                y: 8.0,
                z: 8.0,
            },
        };

        let expected_centre = Vec3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };

        let centre = bounds.get_centre();

        assert_eq!(
            centre, expected_centre,
            "Centre point does not match expected values."
        );
    }

    // TODO: Test for bounds.get_size();

    #[test]
    fn get_split_points() {
        let bounds = Bounds {
            min: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Vec3 {
                x: 8.0,
                y: 8.0,
                z: 8.0,
            },
        };

        let expected_split_points = [0.0, 4.0, 8.0, 0.0, 4.0, 8.0, 0.0, 4.0, 8.0];

        let split_points = Node::get_split_points(&bounds);

        assert_eq!(
            split_points, expected_split_points,
            "Split points do not match expected values."
        );
    }

    #[test]
    fn get_child_bounds() {
        let bounds = Bounds {
            min: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            max: Vec3 {
                x: 8.0,
                y: 8.0,
                z: 8.0,
            },
        };

        let expected_bounds = [
            [0.0, 4.0, 0.0, 4.0, 0.0, 4.0],
            [4.0, 8.0, 0.0, 4.0, 0.0, 4.0],
            [0.0, 4.0, 4.0, 8.0, 0.0, 4.0],
            [4.0, 8.0, 4.0, 8.0, 0.0, 4.0],
            [0.0, 4.0, 0.0, 4.0, 4.0, 8.0],
            [4.0, 8.0, 0.0, 4.0, 4.0, 8.0],
            [0.0, 4.0, 4.0, 8.0, 4.0, 8.0],
            [4.0, 8.0, 4.0, 8.0, 4.0, 8.0],
        ];

        let bounds = Node::get_child_bounds(&bounds);

        assert_eq!(
            bounds, expected_bounds,
            "Split child bounds do not match expected values."
        );
    }
}

use bevy::prelude::*;
use std::sync::{Arc, Mutex};

pub struct NodeBounds {
    pub min: Vec3,
    pub max: Vec3,
}

impl NodeBounds {
    fn unpack(&self) -> [f32; 6] {
        let [min, max] = [self.min, self.max];

        let [xmin, ymin, zmin] = [min.x, min.y, min.z];
        let [xmax, ymax, zmax] = [max.x, max.y, max.z];

        return [xmin, ymin, zmin, xmax, ymax, zmax];
    }

    fn get_centre(&self) -> Vec3 {
        let [xmin, ymin, zmin, xmax, ymax, zmax] = self.unpack();

        return Vec3 {
            x: (xmin + xmax) / 2.0,
            y: (ymin + ymax) / 2.0,
            z: (zmin + zmax) / 2.0,
        };
    }

    fn get_size(&self) -> f32 {
        let [xmin, ymin, zmin, xmax, ymax, zmax] = self.unpack();

        let size = vec![
            ((xmax - xmin) * (xmax - xmin)).sqrt(),
            ((ymax - ymin) * (ymax - ymin)).sqrt(),
            ((zmax - zmin) * (zmax - zmin)).sqrt(),
        ];

        let mut ret = 0.0;
        if let Some(max) = size.iter().cloned().reduce(f32::max) {
            ret = max;
        }

        return ret;
    }
}

enum NodeState {
    UNSPLIT,
    SPLIT,
}

pub struct Node {
    bounds: NodeBounds,
    _child: Leaf,
    children: Vec<Arc<Mutex<Node>>>,
    state: NodeState,
}

impl Node {
    pub fn new(bounds: NodeBounds) -> Node {
        Node {
            bounds,
            _child: Leaf::new(),
            children: vec![],
            state: NodeState::UNSPLIT,
        }
    }

    fn get_split_points(bounds: &NodeBounds) -> [f32; 9] {
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

    fn get_child_bounds(bounds: &NodeBounds) -> [[f32; 6]; 8] {
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

        let new_node = Node::new(NodeBounds {
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
            NodeState::SPLIT => return,
            NodeState::UNSPLIT => {
                let child_bounds = Node::get_child_bounds(&self.bounds);

                for (_, bounds) in child_bounds.iter().enumerate() {
                    self.add_child(*bounds);
                }

                self.state = NodeState::SPLIT;
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
            NodeState::UNSPLIT => return,
            NodeState::SPLIT => {
                self.dispose_children();
                self.state = NodeState::UNSPLIT;
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
    fn unpack_bounds() {
        let bounds = NodeBounds {
            min: Vec3 {
                x: 1.0,
                y: 2.0,
                z: 3.0,
            },
            max: Vec3 {
                x: 4.0,
                y: 5.0,
                z: 6.0,
            },
        };

        let expected_unpack = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0];

        let unpack = bounds.unpack();

        assert_eq!(
            unpack, expected_unpack,
            "Unpacked bound values do not match expected values."
        )
    }

    #[test]
    fn get_centre() {
        let bounds = NodeBounds {
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

    #[test]
    fn get_size() {
        let bounds = NodeBounds {
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

        let expected_size = 8.0;

        let size = bounds.get_size();

        assert!(
            (size - expected_size).abs() < 1e-6,
            "Size does not match expected value."
        );
    }

    #[test]
    fn get_split_points() {
        let bounds = NodeBounds {
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
        let bounds = NodeBounds {
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

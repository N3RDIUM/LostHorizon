use bevy::prelude::*;
use std::sync::{Arc, Mutex};

enum NodeState {
    UNSPLIT,
    SPLIT,
}

pub struct Node {
    bounds: [Vec3; 2],
    child: Leaf,
    children: Vec<Arc<Mutex<Node>>>,
    state: NodeState,
}

impl Node {
    pub fn new(bounds: [Vec3; 2]) -> Node {
        Node {
            bounds,
            child: Leaf {},
            children: vec![],
            state: NodeState::UNSPLIT,
        }
    }

    fn get_split_points(bounds: [Vec3; 2]) -> [f32; 9] {
        let xmin = bounds[0].x;
        let ymin = bounds[0].y;
        let zmin = bounds[0].z;

        let xmax = bounds[1].x;
        let ymax = bounds[1].y;
        let zmax = bounds[1].z;

        let xmid = (xmin + xmax) / 2.0;
        let ymid = (ymin + ymax) / 2.0;
        let zmid = (zmin + zmax) / 2.0;

        return [xmin, xmid, xmax, ymin, ymid, ymax, zmin, zmid, zmax];
    }

    fn get_child_bounds(bounds: [Vec3; 2]) -> [[f32; 6]; 8] {
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

        let new_node = Node::new([
            Vec3 {
                x: xmin,
                y: ymin,
                z: zmin,
            },
            Vec3 {
                x: xmax,
                y: ymax,
                z: zmax,
            },
        ]);

        self.children.push(Arc::new(Mutex::new(new_node)));
    }

    pub fn split(&mut self) {
        match self.state {
            NodeState::SPLIT => return,
            NodeState::UNSPLIT => {
                let child_bounds = Node::get_child_bounds(self.bounds);

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

    pub fn unsplit(&mut self) {
        match self.state {
            NodeState::UNSPLIT => return,
            NodeState::SPLIT => {
                self.dispose_children();
                self.state = NodeState::UNSPLIT;
            }
        }
    }

    fn unpack_bounds(bounds: [Vec3; 2]) -> [f32; 6] {
        let [min, max] = bounds;

        let [xmin, ymin, zmin] = [min.x, min.y, min.z];
        let [xmax, ymax, zmax] = [max.x, max.y, max.z];

        return [xmin, ymin, zmin, xmax, ymax, zmax];
    }

    fn get_centre(bounds: [Vec3; 2]) -> Vec3 {
        let [xmin, ymin, zmin, xmax, ymax, zmax] = Node::unpack_bounds(bounds);

        return Vec3 {
            x: (xmin + xmax) / 2.0,
            y: (ymin + ymax) / 2.0,
            z: (zmin + zmax) / 2.0,
        };
    }

    fn get_size(bounds: [Vec3; 2]) -> f32 {
        let [xmin, ymin, zmin, xmax, ymax, zmax] = Node::unpack_bounds(bounds);

        return (((xmax - xmin) * (xmax - xmin)).sqrt()
            + ((ymax - ymin) * (ymax - ymin)).sqrt()
            + ((zmax - zmin) * (zmax - zmin)).sqrt())
            / 3.0;
    }

    pub fn update(&mut self, cameras: Vec<Vec3>) {
        let centre = Node::get_centre(self.bounds);
        let threshold = Node::get_size(self.bounds);

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

#[cfg(test)]
mod octree_node {
    use super::*;

    #[test]
    fn get_split_points() {
        let bounds = [
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 8.0,
                y: 8.0,
                z: 8.0,
            },
        ];

        let expected_split_points = [0.0, 4.0, 8.0, 0.0, 4.0, 8.0, 0.0, 4.0, 8.0];

        let split_points = Node::get_split_points(bounds);

        assert_eq!(
            split_points, expected_split_points,
            "Split points do not match expected values."
        );
    }

    #[test]
    fn get_child_bounds() {
        let bounds = [
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 8.0,
                y: 8.0,
                z: 8.0,
            },
        ];

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

        let bounds = Node::get_child_bounds(bounds);

        assert_eq!(
            bounds, expected_bounds,
            "Split child bounds do not match expected values."
        );
    }

    #[test]
    fn unpack_bounds() {
        let bounds = [
            Vec3 {
                x: 0.0,
                y: 1.0,
                z: 2.0,
            },
            Vec3 {
                x: 3.0,
                y: 4.0,
                z: 5.0,
            },
        ];

        let expected_unpack = [0.0, 1.0, 2.0, 3.0, 4.0, 5.0];

        let unpack = Node::unpack_bounds(bounds);

        assert_eq!(
            unpack, expected_unpack,
            "Unpacked bound values do not match expected values."
        )
    }

    #[test]
    fn get_centre() {
        let bounds = [
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 8.0,
                y: 8.0,
                z: 8.0,
            },
        ];

        let expected_centre = Vec3 {
            x: 4.0,
            y: 4.0,
            z: 4.0,
        };

        let centre = Node::get_centre(bounds);

        assert_eq!(
            centre, expected_centre,
            "Centre point does not match expected values."
        );
    }

    #[test]
    fn get_size() {
        let bounds = [
            Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
            Vec3 {
                x: 8.0,
                y: 8.0,
                z: 8.0,
            },
        ];

        let expected_size = 8.0;

        let size = Node::get_size(bounds);

        assert!(
            (size - expected_size).abs() < 1e-6,
            "Size does not match expected value."
        );
    }
}

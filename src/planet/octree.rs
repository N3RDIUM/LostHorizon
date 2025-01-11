use bevy::prelude::*;
use std::sync::{Arc, Mutex};

enum NodeState {
    UNSPLIT,
    SPLIT,
}

#[derive(Component)]
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

    pub fn split(mut self) {
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
        self.dispose_children();
        self.state = NodeState::UNSPLIT;
    }
}

#[derive(Component)]
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
}

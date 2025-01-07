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

    pub fn split(mut self) {
        match self.state {
            NodeState::SPLIT => return,
            NodeState::UNSPLIT => {
                let xmin = self.bounds[0].x;
                let ymin = self.bounds[0].y;
                let zmin = self.bounds[0].z;

                let xmax = self.bounds[1].x;
                let ymax = self.bounds[1].y;
                let zmax = self.bounds[1].z;

                let xmid = (xmin + xmax) / 2.0;
                let ymid = (ymin + ymax) / 2.0;
                let zmid = (zmin + zmax) / 2.0;

                let child_bounds = [
                    [xmin, xmid, ymin, ymid, zmin, zmid],
                    [xmid, xmax, ymin, ymid, zmin, zmid],
                    [xmin, xmid, ymid, ymax, zmin, zmid],
                    [xmid, xmax, ymid, ymax, zmin, zmid],
                    [xmin, xmid, ymin, ymid, zmid, zmax],
                    [xmid, xmax, ymin, ymid, zmid, zmax],
                    [xmin, xmid, ymid, ymax, zmid, zmax],
                    [xmid, xmax, ymid, ymax, zmid, zmax],
                ];

                for (_, bounds) in child_bounds.iter().enumerate() {
                    let [xmin, xmax, ymin, ymax, zmin, zmax] = bounds;
                    self.children.push(Arc::new(Mutex::new(Node::new([
                        Vec3 {
                            x: *xmin,
                            y: *ymin,
                            z: *zmin,
                        },
                        Vec3 {
                            x: *xmax,
                            y: *ymax,
                            z: *zmax,
                        },
                    ]))));
                }

                self.state = NodeState::SPLIT;
            }
        }
    }

    pub fn unsplit(&mut self) {
        for child in self.children.iter_mut() {
            let mut node = child.lock().unwrap();
            node.unsplit();
        }
        self.children.clear();
        self.state = NodeState::UNSPLIT;
    }
}

#[derive(Component)]
pub struct Leaf {}


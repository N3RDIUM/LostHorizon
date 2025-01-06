use bevy::prelude::*;

#[derive(Component)]
struct Node {
    position: Vec3,
    size: Vec3,
    children: Vec<Node>,
}


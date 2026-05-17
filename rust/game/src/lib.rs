use godot::prelude::*;

mod constants;
mod team;
mod unit;
mod map_node;
mod game;

struct GodotGdextDemo;

#[gdextension]
unsafe impl ExtensionLibrary for GodotGdextDemo {}

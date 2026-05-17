use godot::prelude::*;

mod hello_node;

// This is the extension entry point. Godot calls gdext_rust_init on load.
struct GodotGdextDemo;

#[gdextension]
unsafe impl ExtensionLibrary for GodotGdextDemo {}

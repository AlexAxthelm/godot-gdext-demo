use godot::classes::{INode2D, Node2D};
use godot::prelude::*;

/// A minimal Node2D subclass written in Rust.
///
/// Drop this onto any scene in the Godot editor to verify the extension
/// is loaded and working. You should see the greeting printed in the
/// Godot output panel when the scene runs.
#[derive(GodotClass)]
#[class(base = Node2D)]
pub struct HelloNode {
    #[export]
    greeting: GString,

    base: Base<Node2D>,
}

#[godot_api]
impl INode2D for HelloNode {
    fn init(base: Base<Node2D>) -> Self {
        Self {
            greeting: "Hello from Rust!".into(),
            base,
        }
    }

    fn ready(&mut self) {
        let msg = self.greeting.clone();
        godot_print!("{msg}");
    }

    fn process(&mut self, delta: f64) {
        // delta is seconds since the last frame — same concept as the Creeps
        // tutorial's _process(delta). Nothing interesting here yet; this is
        // just a placeholder to show the hook exists.
        let _ = delta;
    }
}

#[godot_api]
impl HelloNode {
    /// Called from GDScript as HelloNode.greet("world").
    /// Returns the full greeting string so callers can use it.
    #[func]
    fn greet(&self, name: GString) -> GString {
        format!("{} Nice to meet you, {name}.", self.greeting).as_str().into()
    }
}

// ---------------------------------------------------------------------------
// Pure-Rust unit tests — no Godot runtime needed
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    /// Sanity-check: the greeting format is what we expect.
    /// This runs with plain `cargo test` and does not need Godot.
    #[test]
    fn greet_format() {
        let greeting = "Hello from Rust!";
        let name = "world";
        let result = format!("{greeting} Nice to meet you, {name}.");
        assert_eq!(result, "Hello from Rust! Nice to meet you, world.");
    }
}

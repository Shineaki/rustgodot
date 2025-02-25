use godot::prelude::*;
mod player;
mod server_gd;
mod client_gd;
struct MyExtension;

#[gdextension]
unsafe impl ExtensionLibrary for MyExtension {}

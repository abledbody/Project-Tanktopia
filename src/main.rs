#![warn(clippy::missing_docs_in_private_items)]
//! It's a tank game!

use bevy::prelude::*;
mod menu;

/// Main loop. This really doesn't need an explanation.
fn main() {
    App::build()
		.add_plugins(DefaultPlugins)
		.add_plugin(menu::MenuSystem::new())
		.run();
}
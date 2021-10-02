//! Menu systems

use bevy::prelude::*;

/// A Bevy plugin that implements the menu systems for the game.
pub struct MenuSystem {
	
}

impl Plugin for MenuSystem {
    fn build(&self, app: &mut AppBuilder) {
        
    }
}

impl MenuSystem {
	/// Initializes a new MenuSystem plugin.
	pub fn new() -> Self {
		MenuSystem {}
	}
}
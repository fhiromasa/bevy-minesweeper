use bevy::{app::Plugin, prelude::*};
use resources::tile_map::TileMap;

pub mod components;
pub mod resources;

pub struct BoardPlugin;

impl BoardPlugin {
    fn create_board() {
        let mut tile_map = TileMap::empty(20, 20);
        tile_map.set_bomb(40);

        #[cfg(feature = "debug")]
        log::info!("{}", tile_map.console_output());
    }
}

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::create_board);

        #[cfg(feature = "debug")]
        log::info!("Loaded BoardPlugin");
    }
}

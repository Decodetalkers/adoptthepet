mod animal;
mod human;
mod adored;
mod kitty;
use bevy::prelude::*;
const BACKGROUND_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .run();
    //println!("Hello, world!");
}

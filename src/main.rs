
use bevy::prelude::*;

mod client;
mod server;
mod network_messages;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .run();
}

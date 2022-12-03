use bevy::app::App;
use bevy::{DefaultPlugins, window};
use bevy::prelude::*;
use bevy::sprite::{Sprite, SpriteBundle};
use image_hl::image_hl::Image;

pub fn new_window(){
    App::new().add_plugins(DefaultPlugins)
        /*.add_startup_system(setup)*/
        .run();
}

// fn setup(mut commands: Commands, images: Image){
//     commands.spawn_bundle(Camera2dBundle::default());
//     commands.spawn_bundle(SpriteBundle {
//         texture: images.convert_to_byte_buffer(),
//         ..default()})
// }
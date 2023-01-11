use bevy::app::App;
use bevy::{DefaultPlugins};
use bevy::asset::Assets;
use bevy::input::mouse::{MouseMotion, MouseWheel};
use bevy::math::{Vec2, Vec3};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::scene::SceneInstance;
use bevy::sprite::{ColorMaterial, MaterialMesh2dBundle, };
use bevy::utils::default;
use bevy_inspector_egui::{Inspectable, WorldInspectorPlugin};
use bevy_prototype_debug_lines::DebugLinesPlugin;
use image_hl::image_hl::Imagehl;
use image::{open};


pub fn init_app(){
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_plugin(DebugLinesPlugin::default())
        .add_plugin(WorldInspectorPlugin::default())
        .add_startup_system(setup)
        .add_system(load_image)
        .add_system(pan)
        .add_system(scroll_scale)
        .add_system(check_scroll_bound)
        .add_system(file_drop)
        .run();
}

#[derive(Component,Inspectable,Default)]
struct Camera;
#[derive(Component,Inspectable,Default)]
struct World_Properties {
    num_layers: i8,
}

fn setup(
    mut commands: Commands,
){
    commands.spawn(World {
        id: val,
        entities: val,
        components: val,
        archetypes: val,
        storages: val,
        bundles: val,
        removed_components: val,
        archetype_component_access: val,
        main_thread_validator: val,
        change_tick: val,
        last_change_tick: val });
}

fn load_image(
    mut commands: Commands,
    mut textures: ResMut<Assets<Image>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut query: Query<&mut Scene_Properties, With<Scene_Properties>>
){
    let dynamic_image = open("Message.png").expect("File not found");

    let mut imagehl = Imagehl::new_color_array(dynamic_image);

    let image_width = imagehl.get_width() as f32;
    let image_height = imagehl.get_height() as f32;

    let image = Image::new(
        Extent3d {
                width: imagehl.get_width(),
                height: imagehl.get_height(),
                depth_or_array_layers: 1
            },
        TextureDimension::D2,
        imagehl.convert_to_byte_buffer(),
        TextureFormat::Rgba8Unorm,
    );

    let image_handle = textures.add(image);

    commands.spawn((Camera2dBundle{
        transform: Transform::from_xyz(0.0,0.0,0.0),
        ..default()
        },
        Camera
    ));

    commands.spawn((MaterialMesh2dBundle {
        mesh: meshes.add(Mesh::from(shape::Quad::new(Vec2::new(image_width, image_height)))).into(),
        transform: Transform::default(),
        material: materials.add(ColorMaterial::from(image_handle)),
        ..default()
        },
    ));
}

fn file_drop(
    mut dnd_evr: EventReader<FileDragAndDrop>,
){
    for e in dnd_evr.iter(){
        println!("{:?}",e);
        if let FileDragAndDrop::DroppedFile {id,path_buf} = e {
            println!("Dropped file with path: {:?}, in window id: {:?}", path_buf, id)
        }
    }
}

fn scroll_scale(
    mut scroll_events: EventReader<MouseWheel>,
    mut query: Query<&mut Transform, With<Camera>>,
){
    for e in scroll_events.iter(){
        let scale = Vec3::new(0.1 ,0.1,0.0);
        let mut t = query.single_mut();

        if e.y > 0.0 {
            t.scale -= scale;
        } else if e.y < 0.0 {
            t.scale += scale;
        }
    }
}

fn check_scroll_bound(
    mut query: Query<&mut Transform, With<Camera>>
){
    let mut t = query.single_mut();

    let camera_bound = Vec3::new(0.1,0.1,0.0);

    while t.scale.x < camera_bound.y || t.scale.y < camera_bound.y{
        t.scale += camera_bound;
    }
}

fn pan(
    input: Res<Input<MouseButton>>,
    mut motion_event: EventReader<MouseMotion>,
    mut query: Query<&mut Transform, With<Camera>>
){
    let mut t = query.single_mut();

    if input.pressed(MouseButton::Middle) {
        for e in motion_event.iter() {
            t.translation.x -= e.delta.x * t.scale.x;
            t.translation.y += e.delta.y * t.scale.y;
        }
    }
}



//TODO Global Grid System

//TODO UI Elements

//TODO Drag and Drop fuctionallity

//TODO Tools (paint brush, move tool, effects)

//TODO Color wheel / color picking

//TODO Possibly switch method of zooming
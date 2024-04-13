use bevy::asset::LoadedUntypedAsset;
use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;


#[derive(AssetCollection, Resource)]
pub(crate) struct TestAssets {
    #[asset(path = "torch.png")]
    pub torch: Handle<Image>,
}

#[derive(Resource)]
struct Untyped(Handle<LoadedUntypedAsset>);

fn load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,) {
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(Untyped(asset_server.load_untyped("torch.png")));
}

fn spawn_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // test_assets: Res<TestAssets>,
) {
    commands.spawn(Camera2dBundle::default());

    // commands.spawn(SpriteBundle {
    //     texture: asset_server.load_untyped("torch.png"),
    //     // texture: test_assets.torch.clone(),
    //     transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.)),
    //     ..default()
    // });
}

fn draw(mut commands: Commands,mut ran: Local<bool>, handle: Res<Untyped>, assets: Res<Assets<LoadedUntypedAsset>>) {
    if *ran { return; }
    println!("check");
    if let Some(LoadedUntypedAsset{handle}) = assets.get(&handle.0) {
        commands.spawn(SpriteBundle {
            texture: handle.clone().typed(),
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.)),
            ..default()
        });
        *ran = true;
    }
}

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)

        // .init_collection::<TestAssets>()

        .add_systems(
            Startup,
            load,
        )
        .add_systems(Update, draw)

        .run();
}

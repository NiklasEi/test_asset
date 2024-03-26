use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;
use bevy_asset_loader::prelude::*;


#[derive(AssetCollection, Resource)]
pub(crate) struct TestAssets {
    #[asset(path = "torch.png")]
    pub torch: Handle<Image>,
}

fn spawn_camera(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    test_assets: Res<TestAssets>,
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        // texture: asset_server.load("torch.png"),
        texture: test_assets.torch.clone(),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.)),
        ..default()
    });
}

fn main() {
    let mut app = App::new();
    app
        .add_plugins(DefaultPlugins)

        .init_collection::<TestAssets>()

        .add_systems(
            Startup,
            spawn_camera,
        )

        .run();
}

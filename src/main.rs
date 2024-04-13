use bevy::prelude::*;

#[derive(Resource)]
pub(crate) struct TestAssets {
    pub torch: Handle<Image>,
}

impl FromWorld for TestAssets {
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>();
        TestAssets {
            torch: { asset_server.load("torch.png") },
        }
    }
}

fn spawn_camera(
    mut commands: Commands,
    test_assets: Res<TestAssets>,
    // asset_server: Res<AssetServer>
) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: test_assets.torch.clone(),
        // texture: asset_server.load("torch.png"),
        transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.)),
        ..default()
    });
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .init_resource::<TestAssets>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, check)
        .run();
}

fn check(test_assets: Res<TestAssets>, asset_server: Res<AssetServer>, images: Res<Assets<Image>>) {
    println!("image is in asset collection: {}", images.contains(&test_assets.torch));
    println!("AssetServer reports image in state: {:?}", asset_server.load_state(&test_assets.torch));
}

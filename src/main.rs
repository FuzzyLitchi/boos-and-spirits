use bevy::{prelude::*, render::camera::Camera};

fn main() {
    App::build()
        .add_plugins(DefaultPlugins)
        // .add_plugin(EasingsPlugin)
        .add_plugin(GamePlugin)
        .run();
}

struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup.system())
            .add_system(fit_camera_to_screen.system())
            .add_resource(ClearColor(Color::rgb(0., 0., 0.)));
    }
}

fn setup(
    commands: &mut Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let cards_texture = asset_server.load("cards.png");
    let cards_spritesheet = TextureAtlas::from_grid(cards_texture, Vec2::new(42., 57.), 3, 1);
    texture_atlases.set("cards", cards_spritesheet);

    commands.spawn(Camera2dBundle::default());

    for x in &[31., 80.] {
        for y in &[38., 102., 166.] {
            let pos = Vec2::new(*x, *y) + BOARD_POSITION;

            commands.spawn(SpriteSheetBundle {
                texture_atlas: texture_atlases.get_handle("cards"),
                sprite: TextureAtlasSprite::new(0), // index
                transform: Transform::from_translation(pos.extend(0.1)),
                ..Default::default()
            });
        }
    }

    let background_texture = asset_server.load("background.png");
    commands.spawn(SpriteBundle {
        material: materials.add(background_texture.into()),
        transform: Transform::from_translation(Vec3::new(0., 0., 0.0)),
        ..Default::default()
    });
}

fn fit_camera_to_screen(windows: Res<Windows>, mut query: Query<&mut Transform, With<Camera>>) {
    // Only one camera thanks.
    assert_eq!(query.iter_mut().count(), 1);
    for mut pos in query.iter_mut() {
        match windows.get_primary() {
            Some(window) => {
                let scale = (window.width() / SCREEN_SIZE.x).min(window.height() / SCREEN_SIZE.y);
                pos.scale = Vec2::splat(1.0 / scale).extend(1.0);
            }
            None => debug!("Couldn't get window for camera resizing."),
        }
    }
}

const SCREEN_SIZE: Vec2 = Vec2 { x: 326., y: 205. }; //in pixels
const BOARD_POSITION: Vec2 = Vec2 {
    x: -SCREEN_SIZE.x / 2.,
    y: -SCREEN_SIZE.y / 2.,
};

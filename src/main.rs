// Example of component
#[derive(bevy::ecs::component::Component)]
struct PlayerCameraMarker;

#[derive(bevy::ecs::component::Component)]
struct MeshObject;

// Example of resource
#[derive(bevy::ecs::system::Resource)]
struct GreetTimer(bevy::time::Timer);

fn move_camera(
    time: bevy::ecs::system::Res<bevy::time::Time>,
    key: bevy::ecs::system::Res<bevy::input::Input<bevy::input::keyboard::KeyCode>>,
    mut cameras: bevy::ecs::system::Query<
        (
            &bevy::render::camera::Camera,
            &mut bevy::transform::components::Transform,
        ),
        bevy::ecs::query::With<PlayerCameraMarker>,
    >,
) {
    let speed: f32 = 200.0;
    for (_cam, mut transform) in &mut cameras {
        if key.pressed(bevy::input::keyboard::KeyCode::Left) {
            transform.translation.x = transform.translation.x - speed * time.delta_seconds();
        }
        if key.pressed(bevy::input::keyboard::KeyCode::Right) {
            transform.translation.x = transform.translation.x + speed * time.delta_seconds();
        }
        if key.pressed(bevy::input::keyboard::KeyCode::Up) {
            transform.translation.y = transform.translation.y + speed * time.delta_seconds();
        }
        if key.pressed(bevy::input::keyboard::KeyCode::Down) {
            transform.translation.y = transform.translation.y - speed * time.delta_seconds();
        }
    }
}

fn setup_camera(mut commands: bevy::ecs::system::Commands) {
    commands.spawn((
        bevy::core_pipeline::core_2d::Camera2dBundle {
            transform: bevy::transform::components::Transform::from_xyz(0.0, 0.0, 0.0),
            ..bevy::core_pipeline::core_2d::Camera2dBundle::default()
        },
        PlayerCameraMarker,
    ));
}

fn setup_background(
    mut commands: bevy::ecs::system::Commands,
    asset_server: bevy::ecs::system::ResMut<bevy::asset::AssetServer>,
    mut meshes: bevy::ecs::system::ResMut<bevy::asset::Assets<bevy::render::mesh::Mesh>>,
    // mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let sampler_desc = bevy::render::texture::ImageSamplerDescriptor {
        address_mode_u: bevy::render::texture::ImageAddressMode::Repeat,
        address_mode_v: bevy::render::texture::ImageAddressMode::Repeat,
        ..Default::default()
    };

    let settings = move |s: &mut bevy::render::texture::ImageLoaderSettings| {
        s.sampler = bevy::render::texture::ImageSampler::Descriptor(sampler_desc.clone());
    };

    // let texture_handle = asset_server.load_with_settings("background.png", settings);
    commands.spawn(bevy::sprite::SpriteBundle {
        // material: materials.add(texture_handle.into()),
        sprite: bevy::sprite::Sprite {
            rect: Some(bevy::math::Rect::new(
                -100000.0, -100000.0, 100000.0, 100000.0,
            )),
            ..bevy::sprite::Sprite::default()
        },
        texture: asset_server.load_with_settings("background.png", settings),
        ..bevy::sprite::SpriteBundle::default()
    });
}

fn setup_grid(
    mut commands: bevy::ecs::system::Commands,
    mut meshes: bevy::ecs::system::ResMut<bevy::asset::Assets<bevy::render::mesh::Mesh>>,
    mut materials: bevy::ecs::system::ResMut<bevy::asset::Assets<bevy::sprite::ColorMaterial>>,
) {
    commands.spawn((
        bevy::sprite::MaterialMesh2dBundle {
            mesh: meshes
                .add(bevy::render::mesh::Mesh::from(
                    bevy::prelude::shape::Circle {
                        radius: 1.0,
                        vertices: 50,
                    },
                ))
                .into(),
            transform: bevy::transform::components::Transform::default()
                .with_scale(bevy::math::Vec3::splat(128.0)),
            material: materials.add(bevy::sprite::ColorMaterial::from(
                bevy::render::color::Color::DARK_GREEN,
            )),
            ..bevy::utils::default()
        },
        MeshObject,
    ));
}

// Example of a plugin
pub struct HelpMePlugin;

impl bevy::app::Plugin for HelpMePlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.insert_resource(GreetTimer(bevy::time::Timer::from_seconds(
            2.0,
            bevy::time::TimerMode::Repeating,
        )))
        .add_systems(bevy::app::Startup, (setup_background, setup_grid))
        .add_systems(bevy::app::Update, (move_camera,));
    }
}

fn main() {
    bevy::app::App::new()
        .insert_resource(bevy::core_pipeline::clear_color::ClearColor(
            bevy::render::color::Color::rgb(0.1, 0.0, 0.2),
        ))
        .add_plugins((bevy::DefaultPlugins, HelpMePlugin))
        .add_systems(bevy::app::PreStartup, setup_camera)
        // .add_systems(bevy::app::Startup, add_people)
        // .add_systems(bevy::app::Update, (initial_cli_greeting, greet_people))
        .run();
}

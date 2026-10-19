use bevy::prelude::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.035, 0.055, 0.075)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PokeEmerald-rust | Overworld preview".to_owned(),
                resolution: (1440, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(-13.0, 15.0, 22.0).looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
    ));

    commands.spawn((
        DirectionalLight::default(),
        Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.9, -0.6, 0.0)),
    ));

    let grass = materials.add(Color::srgb(0.19, 0.46, 0.25));
    let earth = materials.add(Color::srgb(0.36, 0.25, 0.16));
    let player_color = materials.add(Color::srgb(0.18, 0.48, 0.92));
    let player_head_color = materials.add(Color::srgb(0.96, 0.76, 0.52));
    let block_mesh = meshes.add(Cuboid::new(1.0, 1.0, 1.0));

    // A procedural block field demonstrates the target visual direction.
    // It is not an imported Pokémon Emerald map.
    for x in -8_i32..=6 {
        for z in -6_i32..=6 {
            let height = 1 + ((x * x + z * z) % 3);
            for y in 0..height {
                let material = if y == height - 1 {
                    grass.clone()
                } else {
                    earth.clone()
                };

                commands.spawn((
                    Mesh3d(block_mesh.clone()),
                    MeshMaterial3d(material),
                    Transform::from_xyz(x as f32, y as f32 + 0.5, z as f32),
                ));
            }
        }
    }

    // Placeholder player: native controls and Pokémon entities are not implemented yet.
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.65, 1.0, 0.65))),
        MeshMaterial3d(player_color),
        Transform::from_xyz(0.0, 1.5, 0.0),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.55, 0.55, 0.55))),
        MeshMaterial3d(player_head_color),
        Transform::from_xyz(0.0, 2.25, 0.0),
    ));

    spawn_companion_panel(&mut commands);
}

fn spawn_companion_panel(commands: &mut Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(0),
                right: px(0),
                width: px(350),
                height: percent(100),
                flex_direction: FlexDirection::Column,
                row_gap: px(13),
                padding: px(24).all(),
                ..default()
            },
            BackgroundColor(Color::srgba(0.035, 0.055, 0.09, 0.96)),
        ))
        .with_children(|panel| {
            panel.spawn((
                Text::new("KANTO GEAR"),
                TextFont {
                    font_size: FontSize::Px(26.0),
                    ..default()
                },
                TextColor(Color::srgb(0.42, 0.91, 0.69)),
            ));
            panel.spawn((
                Text::new("SILPH LINK OS / COMPANION"),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(Color::srgb(0.56, 0.67, 0.72)),
            ));
            panel.spawn((
                Node {
                    width: percent(100),
                    height: px(1),
                    ..default()
                },
                BackgroundColor(Color::srgb(0.18, 0.28, 0.31)),
            ));
            panel.spawn((
                Text::new("TRAINER OVERVIEW"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.95, 0.94)),
            ));
            panel.spawn((
                Text::new(
                    "Region     Hoenn\nLocation   Voxel test field\nParty       Not connected",
                ),
                TextFont {
                    font_size: FontSize::Px(13.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.80, 0.80)),
            ));
            panel.spawn((
                Text::new("QUICK ACCESS"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.95, 0.94)),
            ));
            panel.spawn((
                Text::new(
                    "01  Pokédex\n02  Party\n03  Region map\n04  Bag & tools\n05  Notes & progress",
                ),
                TextFont {
                    font_size: FontSize::Px(15.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.80, 0.80)),
            ));
            panel.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });
            panel.spawn((
                Text::new("PROTOTYPE BUILD\nCompanion data is not connected yet."),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.68, 0.43)),
            ));
        });
}

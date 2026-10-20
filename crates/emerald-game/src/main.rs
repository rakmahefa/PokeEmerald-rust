use bevy::prelude::*;
use emerald_data::{DecodedMapLayout, MapCatalog, MapLayoutCatalog};
use std::{fs, path::PathBuf};

#[derive(Resource, Debug, Clone, Default)]
struct GameDataStatus {
    map_count: usize,
    layout_count: usize,
    source_commit: Option<String>,
    starting_location: Option<String>,
    starting_layout: Option<String>,
    decoded_layout: Option<DecodedMapLayout>,
    bundle_error: Option<String>,
}

impl GameDataStatus {
    fn from_catalog(catalog: MapCatalog) -> Self {
        let starting = catalog.starting_location();
        Self {
            map_count: catalog.map_count,
            layout_count: 0,
            source_commit: Some(catalog.source.commit.clone()),
            starting_location: starting.map(|location| location.name.clone()),
            starting_layout: starting.map(|location| location.layout.clone()),
            decoded_layout: None,
            bundle_error: None,
        }
    }
}

fn main() {
    let data_status = load_map_catalog_status();

    App::new()
        .insert_resource(data_status)
        .insert_resource(ClearColor(Color::srgb(0.035, 0.055, 0.075)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "PokeEmerald-rust | Map layout diagnostics".to_owned(),
                resolution: (1440, 900).into(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_scene)
        .run();
}

fn load_map_catalog_status() -> GameDataStatus {
    let mut asset_directories = Vec::new();
    if let Ok(executable) = std::env::current_exe() {
        if let Some(directory) = executable.parent() {
            asset_directories.push(directory.join("assets"));
        }
    }
    if let Ok(directory) = std::env::current_dir() {
        asset_directories.push(directory.join("assets"));
    }
    asset_directories.dedup();

    let mut last_error =
        "No assets/map_catalog.json found beside the application or in the current directory."
            .to_owned();
    for asset_directory in asset_directories {
        let catalog_path = asset_directory.join("map_catalog.json");
        if !catalog_path.is_file() {
            continue;
        }
        let catalog = match MapCatalog::load(&catalog_path) {
            Ok(catalog) => catalog,
            Err(error) => {
                last_error = format!("Invalid map catalog: {error}");
                eprintln!(
                    "Ignoring invalid map catalog {}: {error}",
                    catalog_path.display()
                );
                continue;
            }
        };

        let mut status = GameDataStatus::from_catalog(catalog);
        let layout_catalog_path = asset_directory.join("map_layouts.json");
        let payload_path = asset_directory.join("map_layouts.bin");
        if !layout_catalog_path.is_file() || !payload_path.is_file() {
            status.bundle_error = Some(
                "Map catalog is present, but map_layouts.json or map_layouts.bin is missing."
                    .to_owned(),
            );
            eprintln!("{}", status.bundle_error.as_deref().unwrap_or_default());
            return status;
        }

        let layout_catalog = match MapLayoutCatalog::load(&layout_catalog_path) {
            Ok(layouts) => layouts,
            Err(error) => {
                status.bundle_error = Some(format!("Invalid map layout catalog: {error}"));
                return status;
            }
        };
        status.layout_count = layout_catalog.layout_count;
        if status.source_commit.as_deref() != Some(layout_catalog.source.commit.as_str()) {
            status.bundle_error =
                Some("Map catalog and layout bundle refer to different source commits.".to_owned());
            return status;
        }

        let starting_layout = match status.starting_layout.as_deref() {
            Some(layout) => layout,
            None => {
                status.bundle_error =
                    Some("No starting layout is declared by the map catalog.".to_owned());
                return status;
            }
        };
        match fs::metadata(&payload_path) {
            Ok(metadata) if metadata.len() == layout_catalog.payload_size => {}
            Ok(metadata) => {
                status.bundle_error = Some(format!(
                    "Layout payload size mismatch: expected {}, found {} bytes.",
                    layout_catalog.payload_size,
                    metadata.len()
                ));
                return status;
            }
            Err(error) => {
                status.bundle_error = Some(format!("Could not inspect layout payload: {error}"));
                return status;
            }
        }
        let payload = match fs::read(&payload_path) {
            Ok(payload) => payload,
            Err(error) => {
                status.bundle_error = Some(format!("Could not read layout payload: {error}"));
                return status;
            }
        };
        match layout_catalog.decode_layout(starting_layout, &payload) {
            Ok(layout) => {
                eprintln!(
                    "Loaded source map layout {} ({}x{}) from pret commit {}.",
                    layout.descriptor.id,
                    layout.descriptor.width,
                    layout.descriptor.height,
                    layout_catalog.source.commit
                );
                status.decoded_layout = Some(layout);
            }
            Err(error) => {
                status.bundle_error =
                    Some(format!("Could not decode starting map layout: {error}"));
                return status;
            }
        }
        return status;
    }

    GameDataStatus {
        bundle_error: Some(last_error),
        ..default()
    }
}

fn setup_scene(mut commands: Commands, data_status: Res<GameDataStatus>) {
    commands.spawn(Camera2d);
    if let Some(layout) = &data_status.decoded_layout {
        spawn_map_layout_diagnostic(&mut commands, layout);
    } else {
        spawn_missing_layout_notice(&mut commands, data_status.bundle_error.as_deref());
    }
}

/// Display actual source metatile IDs from the imported layout bundle.
/// Colours are diagnostic-only; this view does not claim to render game artwork.
fn spawn_map_layout_diagnostic(commands: &mut Commands, layout: &DecodedMapLayout) {
    let width = layout.descriptor.width as f32;
    let height = layout.descriptor.height as f32;
    let tile_size = (760.0 / width.max(height)).clamp(12.0, 64.0);
    let world_width = width * tile_size;
    let world_height = height * tile_size;
    // The companion panel occupies the right side of the window, so shift the
    // map viewport left by half its width to keep the grid visually centred.
    let center_x = -175.0;
    let left = center_x - world_width / 2.0;
    let top = world_height / 2.0;

    commands.spawn((
        Text2d::new(format!(
            "{}  |  {} x {} cells",
            layout.descriptor.name, layout.descriptor.width, layout.descriptor.height
        )),
        TextFont {
            font_size: FontSize::Px(17.0),
            ..default()
        },
        TextColor(Color::srgb(0.91, 0.95, 0.94)),
        Transform::from_xyz(center_x, top + 34.0, 2.0),
    ));

    for (index, cell) in layout.cells.iter().enumerate() {
        let x = (index as u32 % layout.descriptor.width) as f32;
        let y = (index as u32 / layout.descriptor.width) as f32;
        let cell_x = left + (x + 0.5) * tile_size;
        let cell_y = top - (y + 0.5) * tile_size;
        commands.spawn((
            Sprite::from_color(
                diagnostic_metatile_color(cell.metatile_id),
                Vec2::splat((tile_size - 1.5).max(1.0)),
            ),
            Transform::from_xyz(cell_x, cell_y, 0.0),
        ));
        if tile_size >= 20.0 {
            commands.spawn((
                Text2d::new(format!("{:03X}", cell.metatile_id)),
                TextFont {
                    font_size: FontSize::Px((tile_size * 0.23).max(8.0)),
                    ..default()
                },
                TextColor(Color::srgb(0.96, 0.96, 0.88)),
                Transform::from_xyz(cell_x, cell_y, 1.0),
            ));
        }
    }

    commands.spawn((
        Text2d::new(
            "DIAGNOSTIC VIEW — hexadecimal labels are source metatile IDs; colours are not game textures.",
        ),
        TextFont {
            font_size: FontSize::Px(12.0),
            ..default()
        },
        TextColor(Color::srgb(0.68, 0.78, 0.79)),
        Transform::from_xyz(center_x, -top - 28.0, 2.0),
    ));
}

fn diagnostic_metatile_color(metatile_id: u16) -> Color {
    // Stable high-contrast palette to expose repeated/adjacent IDs during
    // layout debugging. The mapping intentionally has no gameplay meaning.
    match metatile_id % 10 {
        0 => Color::srgb(0.19, 0.37, 0.40),
        1 => Color::srgb(0.32, 0.41, 0.27),
        2 => Color::srgb(0.48, 0.36, 0.24),
        3 => Color::srgb(0.35, 0.32, 0.49),
        4 => Color::srgb(0.48, 0.30, 0.35),
        5 => Color::srgb(0.22, 0.43, 0.34),
        6 => Color::srgb(0.47, 0.43, 0.23),
        7 => Color::srgb(0.26, 0.34, 0.49),
        8 => Color::srgb(0.43, 0.39, 0.36),
        _ => Color::srgb(0.34, 0.43, 0.43),
    }
}

fn spawn_missing_layout_notice(commands: &mut Commands, error: Option<&str>) {
    let detail = error.unwrap_or("Map layout bundle not loaded.");
    let message = format!(
        "NO REAL MAP LAYOUT LOADED\n\n{detail}\n\nRun the private build with layout bundle export enabled.\nThe procedural mock overworld has been disabled."
    );
    commands.spawn((
        Text2d::new(message),
        TextFont {
            font_size: FontSize::Px(20.0),
            ..default()
        },
        TextColor(Color::srgb(0.95, 0.71, 0.43)),
        Transform::from_xyz(-175.0, 35.0, 2.0),
    ));
}

fn spawn_companion_panel(commands: &mut Commands, data_status: &GameDataStatus) {
    let start_location = data_status
        .starting_location
        .as_deref()
        .unwrap_or("Unavailable");
    let start_layout = data_status
        .starting_layout
        .as_deref()
        .unwrap_or("Unavailable");
    let source_commit = data_status
        .source_commit
        .as_deref()
        .map(|commit| commit.chars().take(12).collect::<String>())
        .unwrap_or_else(|| "Not loaded".to_owned());
    let data_summary = format!(
        "Maps indexed   {}\nLayouts indexed {}\nSource commit   {}",
        data_status.map_count, data_status.layout_count, source_commit
    );
    let layout_status = match &data_status.decoded_layout {
        Some(layout) => format!(
            "Layout decoded\n{}  ({} x {})",
            layout.descriptor.id, layout.descriptor.width, layout.descriptor.height
        ),
        None => format!(
            "Layout unavailable\n{}",
            data_status
                .bundle_error
                .as_deref()
                .unwrap_or("No validated layout bundle.")
        ),
    };

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
                Text::new("COMPANION SHELL / DEVELOPMENT"),
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
                Text::new("SOURCE DATA"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.95, 0.94)),
            ));
            panel.spawn((
                Text::new(format!(
                    "Region      Hoenn\nLocation    {}\nLayout ID   {}\nParty       Not implemented",
                    start_location, start_layout
                )),
                TextFont {
                    font_size: FontSize::Px(12.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.80, 0.80)),
            ));
            panel.spawn((
                Text::new("DATA BUNDLE"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.95, 0.94)),
            ));
            panel.spawn((
                Text::new(data_summary),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.80, 0.80)),
            ));
            panel.spawn((
                Text::new(layout_status),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(if data_status.decoded_layout.is_some() {
                    Color::srgb(0.51, 0.84, 0.64)
                } else {
                    Color::srgb(0.94, 0.58, 0.43)
                }),
            ));
            panel.spawn((
                Text::new("PLANNED MODULES"),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.95, 0.94)),
            ));
            panel.spawn((
                Text::new(
                    "Pokédex\nParty & summary\nRegion map / Explorer\nBag & field tools\nNotes & progression",
                ),
                TextFont {
                    font_size: FontSize::Px(14.0),
                    ..default()
                },
                TextColor(Color::srgb(0.70, 0.80, 0.80)),
            ));
            panel.spawn(Node {
                flex_grow: 1.0,
                ..default()
            });
            panel.spawn((
                Text::new(if data_status.decoded_layout.is_some() {
                    "REAL LAYOUT DATA LOADED\nDiagnostic palette only; tileset art, collision movement, events and gameplay are still in progress."
                } else {
                    "NO REAL MAP DATA\nThis build will not display a procedural mock as if it were the game."
                }),
                TextFont {
                    font_size: FontSize::Px(11.0),
                    ..default()
                },
                TextColor(Color::srgb(0.91, 0.68, 0.43)),
            ));
        });
}

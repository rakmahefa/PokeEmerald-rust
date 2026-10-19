//! Versioned map-catalog contract and importer for a pinned pret/pokeemerald checkout.
//!
//! This creates a compact index only; it never copies layout binaries, graphics,
//! scripts, generated ROM data, or the ROM itself into the package.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    collections::HashSet,
    error::Error,
    fmt,
    fs,
    path::Path,
};

pub const MAP_CATALOG_SCHEMA_VERSION: u32 = 1;
const MAX_JSON_FILE_SIZE: u64 = 4 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapCatalog {
    pub schema_version: u32,
    pub source: SourceRevision,
    pub map_count: usize,
    pub groups: Vec<MapGroup>,
    pub maps: Vec<MapSummary>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SourceRevision {
    pub project: String,
    pub commit: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapGroup {
    pub id: String,
    pub map_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapSummary {
    pub id: String,
    pub name: String,
    pub group: String,
    pub layout: String,
    pub music: Option<String>,
    pub region_map_section: Option<String>,
    pub weather: Option<String>,
    pub map_type: Option<String>,
    pub battle_scene: Option<String>,
    pub requires_flash: bool,
    pub allow_cycling: bool,
    pub allow_escaping: bool,
    pub allow_running: bool,
    pub show_map_name: bool,
    pub connections: Vec<MapConnection>,
    pub object_event_count: usize,
    pub warp_event_count: usize,
    pub coord_event_count: usize,
    pub bg_event_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MapConnection {
    pub map: String,
    pub offset: i32,
    pub direction: String,
}

#[derive(Debug)]
pub struct DataError(String);

impl DataError {
    fn new(message: impl Into<String>) -> Self {
        Self(message.into())
    }
}

impl fmt::Display for DataError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}

impl Error for DataError {}

impl MapCatalog {
    /// Parse map groups and map metadata from a pinned pret checkout.
    ///
    /// Event scripts and event payloads are not copied; only event counts are kept.
    pub fn import_from_pret(
        source_root: impl AsRef<Path>,
        pret_commit: &str,
    ) -> Result<Self, DataError> {
        if !is_full_commit_hash(pret_commit) {
            return Err(DataError::new(
                "pret commit must be a 40-character hexadecimal commit hash",
            ));
        }

        let map_root = source_root.as_ref().join("data").join("maps");
        let group_index_path = map_root.join("map_groups.json");
        let group_bytes = read_bounded_json(&group_index_path)?;
        let group_index: Value = serde_json::from_slice(&group_bytes).map_err(|error| {
            DataError::new(format!(
                "could not parse {}: {error}",
                group_index_path.display()
            ))
        })?;
        let group_order = group_index
            .get("group_order")
            .and_then(Value::as_array)
            .ok_or_else(|| {
                DataError::new(format!(
                    "{} is missing an array field named group_order",
                    group_index_path.display()
                ))
            })?;
        if group_order.is_empty() {
            return Err(DataError::new("map_groups.json contains no map groups"));
        }

        let mut groups = Vec::with_capacity(group_order.len());
        let mut maps = Vec::new();
        let mut seen_group_ids = HashSet::new();
        let mut seen_map_names = HashSet::new();
        let mut seen_map_ids = HashSet::new();

        for group_value in group_order {
            let group_id = group_value
                .as_str()
                .ok_or_else(|| DataError::new("group_order contains a non-string value"))?;
            validate_path_component(group_id, "map group")?;
            if !seen_group_ids.insert(group_id.to_owned()) {
                return Err(DataError::new(format!(
                    "duplicate map group in group_order: {group_id}"
                )));
            }

            let entries = group_index
                .get(group_id)
                .and_then(Value::as_array)
                .ok_or_else(|| {
                    DataError::new(format!(
                        "map_groups.json is missing an array for group {group_id}"
                    ))
                })?;
            let starting_len = maps.len();

            for map_value in entries {
                let map_name = map_value.as_str().ok_or_else(|| {
                    DataError::new(format!(
                        "map group {group_id} contains a non-string map name"
                    ))
                })?;
                validate_path_component(map_name, "map name")?;
                if !seen_map_names.insert(map_name.to_owned()) {
                    return Err(DataError::new(format!(
                        "map name appears more than once in map_groups.json: {map_name}"
                    )));
                }

                let map_path = map_root.join(map_name).join("map.json");
                let map_bytes = read_bounded_json(&map_path)?;
                let raw: RawMap = serde_json::from_slice(&map_bytes).map_err(|error| {
                    DataError::new(format!("could not parse {}: {error}", map_path.display()))
                })?;
                if raw.name != map_name {
                    return Err(DataError::new(format!(
                        "{} declares map name {:?}, but map_groups.json references {:?}",
                        map_path.display(),
                        raw.name,
                        map_name
                    )));
                }
                if raw.id.trim().is_empty() || raw.layout.trim().is_empty() {
                    return Err(DataError::new(format!(
                        "{} must declare non-empty id and layout fields",
                        map_path.display()
                    )));
                }
                if !seen_map_ids.insert(raw.id.clone()) {
                    return Err(DataError::new(format!(
                        "duplicate map id in source data: {}",
                        raw.id
                    )));
                }

                maps.push(MapSummary {
                    id: raw.id,
                    name: raw.name,
                    group: group_id.to_owned(),
                    layout: raw.layout,
                    music: raw.music,
                    region_map_section: raw.region_map_section,
                    weather: raw.weather,
                    map_type: raw.map_type,
                    battle_scene: raw.battle_scene,
                    requires_flash: raw.requires_flash,
                    allow_cycling: raw.allow_cycling,
                    allow_escaping: raw.allow_escaping,
                    allow_running: raw.allow_running,
                    show_map_name: raw.show_map_name,
                    connections: raw.connections,
                    object_event_count: raw.object_events.len(),
                    warp_event_count: raw.warp_events.len(),
                    coord_event_count: raw.coord_events.len(),
                    bg_event_count: raw.bg_events.len(),
                });
            }

            groups.push(MapGroup {
                id: group_id.to_owned(),
                map_count: maps.len() - starting_len,
            });
        }

        let catalog = Self {
            schema_version: MAP_CATALOG_SCHEMA_VERSION,
            source: SourceRevision {
                project: "pret/pokeemerald".to_owned(),
                commit: pret_commit.to_ascii_lowercase(),
            },
            map_count: maps.len(),
            groups,
            maps,
        };
        catalog.validate()?;
        Ok(catalog)
    }

    /// Load a generated catalog and validate its schema before use.
    pub fn load(path: impl AsRef<Path>) -> Result<Self, DataError> {
        let path = path.as_ref();
        let bytes = read_bounded_json(path)?;
        let catalog: Self = serde_json::from_slice(&bytes).map_err(|error| {
            DataError::new(format!(
                "could not parse map catalog {}: {error}",
                path.display()
            ))
        })?;
        catalog.validate()?;
        Ok(catalog)
    }

    pub fn validate(&self) -> Result<(), DataError> {
        if self.schema_version != MAP_CATALOG_SCHEMA_VERSION {
            return Err(DataError::new(format!(
                "unsupported map catalog schema {}; expected {}",
                self.schema_version, MAP_CATALOG_SCHEMA_VERSION
            )));
        }
        if self.source.project != "pret/pokeemerald" {
            return Err(DataError::new("map catalog source project is not pret/pokeemerald"));
        }
        if !is_full_commit_hash(&self.source.commit) {
            return Err(DataError::new("map catalog source commit is invalid"));
        }
        if self.map_count != self.maps.len() {
            return Err(DataError::new(format!(
                "map catalog declares {} maps but contains {} records",
                self.map_count,
                self.maps.len()
            )));
        }
        if self.maps.is_empty() || self.groups.is_empty() {
            return Err(DataError::new("map catalog has no map records or groups"));
        }

        let declared_group_count: usize = self.groups.iter().map(|group| group.map_count).sum();
        if declared_group_count != self.map_count {
            return Err(DataError::new(format!(
                "map group counts sum to {declared_group_count}, expected {}",
                self.map_count
            )));
        }

        let mut group_ids = HashSet::new();
        for group in &self.groups {
            if group.id.trim().is_empty() || !group_ids.insert(group.id.clone()) {
                return Err(DataError::new("map catalog has an empty or duplicate group id"));
            }
        }

        let mut map_ids = HashSet::new();
        let mut map_names = HashSet::new();
        for map in &self.maps {
            if map.id.trim().is_empty()
                || map.name.trim().is_empty()
                || map.layout.trim().is_empty()
                || !group_ids.contains(&map.group)
            {
                return Err(DataError::new(format!(
                    "map catalog contains incomplete metadata for {:?}",
                    map.name
                )));
            }
            if !map_ids.insert(map.id.clone()) || !map_names.insert(map.name.clone()) {
                return Err(DataError::new(format!(
                    "map catalog contains duplicate id or name for {:?}",
                    map.name
                )));
            }
        }
        Ok(())
    }

    /// Resolve the intended starting room if this source revision includes it.
    pub fn starting_location(&self) -> Option<&MapSummary> {
        self.maps
            .iter()
            .find(|map| map.name == "LittlerootTown_MaysHouse_2F")
            .or_else(|| self.maps.first())
    }

    /// Write a stable JSON file and create its parent directory if needed.
    pub fn write_json(&self, path: impl AsRef<Path>) -> Result<(), DataError> {
        self.validate()?;
        let path = path.as_ref();
        if let Some(parent) = path.parent().filter(|parent| !parent.as_os_str().is_empty()) {
            fs::create_dir_all(parent).map_err(|error| {
                DataError::new(format!(
                    "could not create output directory {}: {error}",
                    parent.display()
                ))
            })?;
        }
        let mut json = serde_json::to_vec_pretty(self)
            .map_err(|error| DataError::new(format!("could not serialize map catalog: {error}")))?;
        json.push(b'\n');
        fs::write(path, json).map_err(|error| {
            DataError::new(format!("could not write map catalog {}: {error}", path.display()))
        })
    }
}

#[derive(Debug, Deserialize)]
struct RawMap {
    id: String,
    name: String,
    layout: String,
    #[serde(default)]
    music: Option<String>,
    #[serde(default)]
    region_map_section: Option<String>,
    #[serde(default)]
    weather: Option<String>,
    #[serde(default)]
    map_type: Option<String>,
    #[serde(default)]
    battle_scene: Option<String>,
    #[serde(default)]
    requires_flash: bool,
    #[serde(default)]
    allow_cycling: bool,
    #[serde(default)]
    allow_escaping: bool,
    #[serde(default)]
    allow_running: bool,
    #[serde(default)]
    show_map_name: bool,
    #[serde(default)]
    connections: Vec<MapConnection>,
    #[serde(default)]
    object_events: Vec<Value>,
    #[serde(default)]
    warp_events: Vec<Value>,
    #[serde(default)]
    coord_events: Vec<Value>,
    #[serde(default)]
    bg_events: Vec<Value>,
}

fn read_bounded_json(path: &Path) -> Result<Vec<u8>, DataError> {
    let metadata = fs::metadata(path)
        .map_err(|error| DataError::new(format!("could not read {}: {error}", path.display())))?;
    if metadata.len() > MAX_JSON_FILE_SIZE {
        return Err(DataError::new(format!(
            "refusing oversized JSON file {} ({} bytes)",
            path.display(),
            metadata.len()
        )));
    }
    fs::read(path).map_err(|error| {
        DataError::new(format!("could not read {}: {error}", path.display()))
    })
}

fn validate_path_component(value: &str, label: &str) -> Result<(), DataError> {
    if value.is_empty()
        || !value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
    {
        return Err(DataError::new(format!(
            "unsafe or invalid {label} path component: {value:?}"
        )));
    }
    Ok(())
}

fn is_full_commit_hash(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        path::PathBuf,
        time::{SystemTime, UNIX_EPOCH},
    };

    fn fixture_root(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("clock after epoch")
            .as_nanos();
        let root = std::env::temp_dir().join(format!(
            "emerald-data-{label}-{}-{nonce}",
            std::process::id()
        ));
        let maps = root.join("data/maps");
        fs::create_dir_all(maps.join("Route101")).expect("create fixture maps");
        fs::write(
            maps.join("map_groups.json"),
            r#"{"group_order":["gMapGroup_Test"],"gMapGroup_Test":["Route101"]}"#,
        )
        .expect("write map group fixture");
        fs::write(
            maps.join("Route101/map.json"),
            r#"{
                "id":"MAP_ROUTE101",
                "name":"Route101",
                "layout":"LAYOUT_ROUTE101",
                "music":"MUS_ROUTE101",
                "region_map_section":"MAPSEC_ROUTE_101",
                "requires_flash":false,
                "weather":"WEATHER_SUNNY",
                "map_type":"MAP_TYPE_ROUTE",
                "allow_cycling":true,
                "allow_escaping":false,
                "allow_running":true,
                "show_map_name":true,
                "battle_scene":"MAP_BATTLE_SCENE_NORMAL",
                "connections":[{"map":"MAP_OLDALE_TOWN","offset":0,"direction":"up"}],
                "object_events":[{},{}],
                "warp_events":[],
                "coord_events":[{}],
                "bg_events":[{},{}]
            }"#,
        )
        .expect("write map fixture");
        root
    }

    #[test]
    fn imports_map_metadata_and_event_counts() {
        let root = fixture_root("counts");
        let catalog = MapCatalog::import_from_pret(&root, &"a".repeat(40)).expect("import catalog");
        assert_eq!(catalog.map_count, 1);
        assert_eq!(catalog.groups[0].id, "gMapGroup_Test");
        assert_eq!(catalog.maps[0].name, "Route101");
        assert_eq!(catalog.maps[0].connections[0].map, "MAP_OLDALE_TOWN");
        assert_eq!(catalog.maps[0].object_event_count, 2);
        assert_eq!(catalog.maps[0].coord_event_count, 1);
        assert_eq!(catalog.maps[0].bg_event_count, 2);
        assert!(catalog.maps[0].allow_cycling);
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn catalog_json_round_trips_through_validating_loader() {
        let root = fixture_root("roundtrip");
        let catalog = MapCatalog::import_from_pret(&root, &"b".repeat(40)).expect("import catalog");
        let output = root.join("generated/assets/map_catalog.json");
        catalog.write_json(&output).expect("write catalog");
        let loaded = MapCatalog::load(&output).expect("load catalog");
        assert_eq!(loaded, catalog);
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn rejects_invalid_commit_hash() {
        let root = fixture_root("hash");
        let result = MapCatalog::import_from_pret(&root, "not-a-commit");
        assert!(result.is_err());
        fs::remove_dir_all(root).expect("remove fixture");
    }

    #[test]
    fn rejects_missing_map_metadata() {
        let root = fixture_root("missing");
        fs::remove_file(root.join("data/maps/Route101/map.json")).expect("remove map fixture");
        let result = MapCatalog::import_from_pret(&root, &"c".repeat(40));
        assert!(result.is_err());
        fs::remove_dir_all(root).expect("remove fixture");
    }
}
